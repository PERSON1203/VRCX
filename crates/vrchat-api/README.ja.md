# vrchat-api

VRCXプロジェクトのJavaScript APIレイヤーから生成された、[VRChat API](https://api.vrchat.cloud/api/1)向けの非同期Rustクライアントクレートです。

## 機能

- Cookieベースのセッション管理（手動でのトークン管理不要）
- 強く型付けされたリクエストパラメータとレスポンスモデル
- VRCXが使用するすべてのAPIドメインに対応：
  - **Auth** – ログイン、2FA（OTP / TOTP / メールOTP）、設定
  - **User** – ユーザーの取得／検索／更新、タグ、メモ、共通フレンド／グループ、boop
  - **Avatar** – アバターの取得／検索／保存／選択／削除、インポスター、ギャラリー、スタイル
  - **Avatar moderation** – アバター外見のブロック／ブロック解除
  - **World** – ワールドの取得／検索／保存／削除／公開
  - **Instance** – インスタンスの取得／作成、ショートネーム、自己招待
  - **Friend** – フレンド一覧、フレンドリクエストの送信／キャンセル／削除、フレンドステータス
  - **Favorite** – お気に入りとお気に入りグループの管理
  - **Group** – グループのCRUD全般、メンバー、ロール、招待、参加リクエスト、BAN、投稿、ギャラリー、監査ログ
  - **Calendar** – グループイベントの作成／更新／削除／フォロー
  - **Notification** – v1 & v2 通知、招待の送信／応答
  - **Player moderation** – プレイヤーのブロック／ミュート／ミュート解除
  - **Invite messages** – 招待メッセージスロットの一覧表示と編集
  - **Inventory** – アイテム、テンプレート、装備、アーカイブ、報酬
  - **File** – メタデータ、アップロードライフサイクル（開始／完了）
  - **Misc** – クレジット残高、ワールドの永続データ、バッジ、訪問、ユーザー報告、ファイル解析
  - **Prop** – プロップ情報の取得
  - **Print** – プリントの一覧表示／取得／削除

## 使い方

`Cargo.toml` に以下を追加してください：

```toml
[dependencies]
vrchat-api = { path = "../crates/vrchat-api" }
tokio = { version = "1", features = ["full"] }
```

### 基本的な使用例

```rust
use vrchat_api::{VRChatClient, api::friend::GetFriendsParams};

#[tokio::main]
async fn main() -> vrchat_api::Result<()> {
    let client = VRChatClient::new()?;

    // ログイン（セッションCookieは自動的に保存されます）
    let me = client.login("your_username", "your_password").await?;
    println!("ログインしました: {}", me.display_name);

    // 2FAが必要な場合:
    // client.verify_totp("123456").await?;

    // オンラインのフレンドを取得
    let friends = client
        .get_friends(&GetFriendsParams {
            n: Some(100),
            offline: Some(false),
            ..Default::default()
        })
        .await?;
    println!("オンラインのフレンド: {}人", friends.len());

    Ok(())
}
```

## アーキテクチャ

```
src/
├── lib.rs              # クレートのルート、再エクスポート
├── client.rs           # VRChatClient + HTTPヘルパー (get/post/put/delete)
├── error.rs            # VRChatError 列挙型
├── models/             # 強く型付けされたレスポンス構造体
│   ├── common.rs       # UnityPackage, FileRecord, …
│   ├── user.rs         # User, CurrentUser, FriendStatus, …
│   ├── avatar.rs       # Avatar, AvatarStyle, AvatarModeration
│   ├── world.rs        # World
│   ├── group.rs        # Group, GroupMember, GroupRole, GroupPost, CalendarEvent, …
│   ├── instance.rs     # Instance, InstanceShortName
│   ├── notification.rs # Notification, InviteMessage, PlayerModeration
│   ├── favorite.rs     # Favorite, FavoriteGroup
│   └── inventory.rs    # InventoryItem, InventoryTemplate, VRChatBalance, Print, Prop
└── api/                # APIメソッドの実装（すべて VRChatClient に impl）
    ├── auth.rs
    ├── user.rs
    ├── avatar.rs
    ├── avatar_moderation.rs
    ├── world.rs
    ├── instance.rs
    ├── friend.rs
    ├── favorite.rs
    ├── group.rs        # カレンダーイベントも含む
    ├── notification.rs
    ├── player_moderation.rs
    ├── invite_messages.rs
    ├── inventory.rs
    ├── file.rs
    ├── misc.rs
    ├── prop.rs
    └── print.rs
```

## 認証

VRChatは初回ログインにHTTP Basic認証を使用し、以降のリクエストには永続的なセッションCookieを使用します。
`VRChatClient` は `reqwest` のCookieストアを有効にした状態で構築されているため、セッションは自動的に維持されます。

```
GET /auth/user          ← Basic認証 (username:password)
POST /auth/twofactorauth/totp/verify  ← 2FAが必要な場合
... 以降のリクエストはサーバーが設定した "auth" Cookieを使用
```

## ライセンス

MIT
