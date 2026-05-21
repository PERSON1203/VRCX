# vrchat-api

An async Rust client crate for the [VRChat API](https://api.vrchat.cloud/api/1), generated from the VRCX project's JavaScript API layer.

## Features

- Full cookie-based session management (no manual token handling)
- Strongly-typed request params and response models
- Covers all API domains used by VRCX:
  - **Auth** – login, 2FA (OTP / TOTP / email OTP), config
  - **User** – get/search/update users, tags, notes, mutual friends/groups, boop
  - **Avatar** – get/search/save/select/delete avatars, impostors, gallery, styles
  - **Avatar moderation** – block/unblock avatar appearances
  - **World** – get/search/save/delete/publish worlds
  - **Instance** – get/create instances, short names, self-invite
  - **Friend** – list friends, send/cancel/delete friend requests, friend status
  - **Favorite** – manage favorites and favorite groups
  - **Group** – full group CRUD, members, roles, invites, join requests, bans, posts, gallery, audit logs
  - **Calendar** – create/update/delete/follow group events
  - **Notification** – v1 & v2 notifications, invite send/respond
  - **Player moderation** – block/mute/unmute players
  - **Invite messages** – list and edit invite message slots
  - **Inventory** – items, templates, equip, archive, rewards
  - **File** – metadata, upload lifecycle (start/finish)
  - **Misc** – credits balance, world persist data, badges, visits, report user, file analysis
  - **Prop** – get prop info
  - **Print** – list/get/delete prints

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
vrchat-api = { path = "../crates/vrchat-api" }
tokio = { version = "1", features = ["full"] }
```

### Basic example

```rust
use vrchat_api::{VRChatClient, api::friend::GetFriendsParams};

#[tokio::main]
async fn main() -> vrchat_api::Result<()> {
    let client = VRChatClient::new()?;

    // Login (stores session cookie automatically)
    let me = client.login("your_username", "your_password").await?;
    println!("Logged in as: {}", me.display_name);

    // If 2FA is required:
    // client.verify_totp("123456").await?;

    // Fetch online friends
    let friends = client
        .get_friends(&GetFriendsParams {
            n: Some(100),
            offline: Some(false),
            ..Default::default()
        })
        .await?;
    println!("{} online friends", friends.len());

    Ok(())
}
```

## Architecture

```
src/
├── lib.rs              # crate root, re-exports
├── client.rs           # VRChatClient + HTTP helpers (get/post/put/delete)
├── error.rs            # VRChatError enum
├── models/             # Strongly-typed response structs
│   ├── common.rs       # UnityPackage, FileRecord, …
│   ├── user.rs         # User, CurrentUser, FriendStatus, …
│   ├── avatar.rs       # Avatar, AvatarStyle, AvatarModeration
│   ├── world.rs        # World
│   ├── group.rs        # Group, GroupMember, GroupRole, GroupPost, CalendarEvent, …
│   ├── instance.rs     # Instance, InstanceShortName
│   ├── notification.rs # Notification, InviteMessage, PlayerModeration
│   ├── favorite.rs     # Favorite, FavoriteGroup
│   └── inventory.rs    # InventoryItem, InventoryTemplate, VRChatBalance, Print, Prop
└── api/                # API method implementations (all impl VRChatClient)
    ├── auth.rs
    ├── user.rs
    ├── avatar.rs
    ├── avatar_moderation.rs
    ├── world.rs
    ├── instance.rs
    ├── friend.rs
    ├── favorite.rs
    ├── group.rs        # also covers calendar events
    ├── notification.rs
    ├── player_moderation.rs
    ├── invite_messages.rs
    ├── inventory.rs
    ├── file.rs
    ├── misc.rs
    ├── prop.rs
    └── print.rs
```

## Authentication

VRChat uses HTTP Basic Auth for the initial login, then a persistent session cookie for subsequent requests.
`VRChatClient` is built with `reqwest`'s cookie store enabled, so the session is maintained automatically.

```
GET /auth/user          ← Basic auth (username:password)
POST /auth/twofactorauth/totp/verify  ← if 2FA required
... subsequent requests use the "auth" cookie set by the server
```

## License

MIT
