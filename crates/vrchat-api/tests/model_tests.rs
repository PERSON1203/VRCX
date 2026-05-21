//! Serde round-trip tests for the public model types.

use vrchat_api::models::{
    Avatar, AvatarModeration, AvatarStyle, AvatarStyles, Badge, CurrentUser, Favorite,
    FavoriteGroup, FriendStatus, Instance, InstancePlatforms, MutualCounts, PastDisplayName,
    UnityPackage, User, UserNote, UserPresence, World,
};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Deserialise JSON, re-serialise to JSON, then deserialise again and compare
/// the intermediate JSON values.  This ensures we round-trip without loss.
fn round_trip<T>(json: &str)
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    let value: serde_json::Value = serde_json::from_str(json).expect("invalid JSON fixture");
    let typed: T = serde_json::from_value(value.clone()).expect("failed to deserialise into T");
    let back = serde_json::to_value(&typed).expect("failed to re-serialise T");
    // Only compare keys present in the original fixture; unknown/extra keys
    // may be captured in `#[serde(flatten)] extra` maps.
    for (k, v) in value.as_object().unwrap() {
        assert_eq!(
            back.get(k),
            Some(v),
            "round-trip mismatch for key `{k}`"
        );
    }
}

// ---------------------------------------------------------------------------
// User
// ---------------------------------------------------------------------------

#[test]
fn user_round_trip() {
    let json = r#"{
        "id": "usr_abc",
        "displayName": "TestUser",
        "bio": "Hello",
        "bioLinks": ["https://example.com"],
        "currentAvatarImageUrl": "https://img.example.com/avatar.png",
        "currentAvatarThumbnailImageUrl": "https://img.example.com/thumb.png",
        "currentAvatarTags": [],
        "developerType": "none",
        "isFriend": false,
        "lastActivity": "2024-01-01T00:00:00.000Z",
        "lastLogin": "2024-01-01T00:00:00.000Z",
        "lastPlatform": "standalonewindows",
        "profilePicOverride": "",
        "profilePicOverrideThumbnail": "",
        "pronouns": "they/them",
        "state": "offline",
        "status": "active",
        "statusDescription": "",
        "tags": ["system_trust_veteran"],
        "userIcon": "",
        "ageVerificationStatus": "hidden",
        "ageVerified": false,
        "allowAvatarCopying": true,
        "dateJoined": "2020-06-15",
        "discordId": "",
        "friendKey": "key_abc",
        "badges": []
    }"#;
    round_trip::<User>(json);
}

#[test]
fn user_optional_fields_absent() {
    let json = r#"{
        "id": "usr_minimal",
        "displayName": "Min",
        "bio": "",
        "bioLinks": [],
        "currentAvatarImageUrl": "",
        "currentAvatarThumbnailImageUrl": "",
        "currentAvatarTags": [],
        "developerType": "none",
        "isFriend": false,
        "lastActivity": "",
        "lastLogin": "",
        "lastPlatform": "standalonewindows",
        "profilePicOverride": "",
        "profilePicOverrideThumbnail": "",
        "pronouns": "",
        "state": "offline",
        "status": "active",
        "statusDescription": "",
        "tags": [],
        "userIcon": "",
        "ageVerificationStatus": "hidden",
        "ageVerified": false,
        "allowAvatarCopying": false,
        "dateJoined": "2021-01-01",
        "discordId": "",
        "friendKey": "",
        "badges": []
    }"#;
    let user: User = serde_json::from_str(json).unwrap();
    assert_eq!(user.id, "usr_minimal");
    assert!(user.last_mobile.is_none());
    assert!(user.location.is_none());
    assert!(user.note.is_none());
}

// ---------------------------------------------------------------------------
// Badge
// ---------------------------------------------------------------------------

#[test]
fn badge_round_trip() {
    let json = r#"{
        "badgeId": "bdg_1",
        "badgeName": "Early Supporter",
        "badgeDescription": "Joined early",
        "badgeImageUrl": "https://img.example.com/badge.png",
        "showcased": true
    }"#;
    round_trip::<Badge>(json);
}

// ---------------------------------------------------------------------------
// CurrentUser
// ---------------------------------------------------------------------------

#[test]
fn current_user_deserialise() {
    let json = r#"{
        "id": "usr_me",
        "displayName": "Me",
        "bio": "",
        "bioLinks": [],
        "currentAvatarImageUrl": "",
        "currentAvatarThumbnailImageUrl": "",
        "currentAvatarTags": [],
        "developerType": "none",
        "isFriend": false,
        "lastActivity": "",
        "lastLogin": "",
        "lastPlatform": "standalonewindows",
        "profilePicOverride": "",
        "profilePicOverrideThumbnail": "",
        "pronouns": "",
        "state": "online",
        "status": "active",
        "statusDescription": "",
        "tags": [],
        "userIcon": "",
        "ageVerificationStatus": "hidden",
        "ageVerified": false,
        "allowAvatarCopying": false,
        "dateJoined": "2021-01-01",
        "discordId": "",
        "friendKey": "",
        "badges": [],
        "acceptedPrivacyVersion": 1,
        "acceptedTosVersion": 8,
        "activeFriends": ["usr_a", "usr_b"],
        "currentAvatar": "avtr_xyz",
        "emailVerified": true,
        "fallbackAvatar": "avtr_fallback",
        "friendGroupNames": [],
        "friends": ["usr_a"],
        "hasBirthday": true,
        "hasEmail": true,
        "hasLoggedInFromClient": true,
        "hasPendingEmail": false,
        "homeLocation": "wrld_home",
        "isAdult": true,
        "isBoopingEnabled": true,
        "obfuscatedEmail": "t***@example.com",
        "obfuscatedPendingEmail": "",
        "oculusId": "",
        "offlineFriends": [],
        "onlineFriends": ["usr_a"],
        "pastDisplayNames": [],
        "picoId": "",
        "presence": null
    }"#;
    let cu: CurrentUser = serde_json::from_str(json).unwrap();
    assert_eq!(cu.id, "usr_me");
    assert_eq!(cu.display_name, "Me");
    assert!(cu.email_verified);
    assert_eq!(cu.active_friends, vec!["usr_a", "usr_b"]);
    assert!(cu.presence.is_none());
}

// ---------------------------------------------------------------------------
// UserPresence
// ---------------------------------------------------------------------------

#[test]
fn user_presence_round_trip() {
    let json = r#"{
        "id": "usr_presence",
        "displayName": "PresenceUser",
        "status": "active",
        "world": "wrld_abc",
        "instance": "wrld_abc:12345",
        "instanceType": "public",
        "platform": "standalonewindows",
        "avatarThumbnail": "https://img.example.com/thumb.png",
        "profilePicOverride": "",
        "userIcon": "",
        "currentAvatarTags": "",
        "debugFlag": "",
        "groups": [],
        "travelingToWorld": "",
        "travelingToInstance": ""
    }"#;
    round_trip::<UserPresence>(json);
}

// ---------------------------------------------------------------------------
// PastDisplayName
// ---------------------------------------------------------------------------

#[test]
fn past_display_name_round_trip() {
    let json = r#"{"displayName": "OldName", "updatedAt": "2022-06-01T00:00:00.000Z"}"#;
    round_trip::<PastDisplayName>(json);
}

// ---------------------------------------------------------------------------
// FriendStatus
// ---------------------------------------------------------------------------

#[test]
fn friend_status_round_trip() {
    let json = r#"{"isFriend": true, "outgoingRequest": false, "incomingRequest": false}"#;
    let fs: FriendStatus = serde_json::from_str(json).unwrap();
    assert!(fs.is_friend);
    assert!(!fs.outgoing_request);
    assert!(!fs.incoming_request);
}

// ---------------------------------------------------------------------------
// MutualCounts
// ---------------------------------------------------------------------------

#[test]
fn mutual_counts_round_trip() {
    let json = r#"{"mutualFriendCount": 3, "mutualGroupCount": 1}"#;
    let mc: MutualCounts = serde_json::from_str(json).unwrap();
    assert_eq!(mc.mutual_friend_count, 3);
    assert_eq!(mc.mutual_group_count, 1);
}

// ---------------------------------------------------------------------------
// UserNote
// ---------------------------------------------------------------------------

#[test]
fn user_note_round_trip() {
    let json = r#"{
        "id": "note_1",
        "userId": "usr_me",
        "targetUserId": "usr_them",
        "note": "Met at VRC meetup",
        "createdAt": "2024-01-01T00:00:00.000Z",
        "updatedAt": "2024-01-02T00:00:00.000Z"
    }"#;
    round_trip::<UserNote>(json);
}

// ---------------------------------------------------------------------------
// World
// ---------------------------------------------------------------------------

#[test]
fn world_round_trip() {
    let json = r#"{
        "id": "wrld_abc",
        "name": "Cool World",
        "authorId": "usr_author",
        "authorName": "AuthorName",
        "description": "A cool world",
        "imageUrl": "https://img.example.com/world.png",
        "thumbnailImageUrl": "https://img.example.com/world_thumb.png",
        "createdAt": "2021-01-01T00:00:00.000Z",
        "updatedAt": "2024-01-01T00:00:00.000Z",
        "releaseStatus": "public",
        "tags": ["author_tag_featured"],
        "featured": false,
        "unityPackages": [],
        "capacity": 32,
        "recommendedCapacity": 16,
        "favorites": 100,
        "heat": 5,
        "popularity": 10,
        "publicationDate": "2021-01-01",
        "labsPublicationDate": "none",
        "organization": "vrchat",
        "udonProducts": []
    }"#;
    let world: World = serde_json::from_str(json).unwrap();
    assert_eq!(world.id, "wrld_abc");
    assert_eq!(world.capacity, 32);
    assert!(!world.featured);
}

// ---------------------------------------------------------------------------
// Avatar
// ---------------------------------------------------------------------------

#[test]
fn avatar_round_trip() {
    let json = r#"{
        "id": "avtr_abc",
        "name": "Cool Avatar",
        "authorId": "usr_author",
        "authorName": "AuthorName",
        "description": "A cool avatar",
        "imageUrl": "https://img.example.com/avatar.png",
        "thumbnailImageUrl": "https://img.example.com/avatar_thumb.png",
        "createdAt": "2021-01-01T00:00:00.000Z",
        "updatedAt": "2024-01-01T00:00:00.000Z",
        "releaseStatus": "public",
        "tags": [],
        "featured": false,
        "unityPackages": [],
        "version": 1,
        "unityPackageUrl": "",
        "searchable": true,
        "pendingUpload": false
    }"#;
    let avatar: Avatar = serde_json::from_str(json).unwrap();
    assert_eq!(avatar.id, "avtr_abc");
    assert_eq!(avatar.version, 1);
    assert!(avatar.searchable);
}

// ---------------------------------------------------------------------------
// AvatarStyle
// ---------------------------------------------------------------------------

#[test]
fn avatar_style_round_trip() {
    let json = r#"{"id": "style_1", "name": "Chibi", "description": "Cute chibi style"}"#;
    round_trip::<AvatarStyle>(json);
}

#[test]
fn avatar_styles_struct_round_trip() {
    let json = r#"{"primary": "style_1", "secondary": null}"#;
    let s: AvatarStyles = serde_json::from_str(json).unwrap();
    assert_eq!(s.primary.as_deref(), Some("style_1"));
    assert!(s.secondary.is_none());
}

// ---------------------------------------------------------------------------
// AvatarModeration
// ---------------------------------------------------------------------------

#[test]
fn avatar_moderation_round_trip() {
    let json = r#"{
        "id": "avatarmod_1",
        "sourceUserId": "usr_reporter",
        "targetAvatarId": "avtr_bad",
        "avatarModerationType": "avatar_flag",
        "created": "2024-01-01T00:00:00.000Z"
    }"#;
    round_trip::<AvatarModeration>(json);
}

// ---------------------------------------------------------------------------
// UnityPackage
// ---------------------------------------------------------------------------

#[test]
fn unity_package_round_trip() {
    let json = r#"{
        "id": "unitypackage_abc",
        "assetVersion": 3,
        "createdAt": "2021-06-01T00:00:00.000Z",
        "platform": "standalonewindows",
        "unityVersion": "2019.4.31f1",
        "variant": "standard",
        "assetUrl": "https://cdn.example.com/asset.vrca"
    }"#;
    round_trip::<UnityPackage>(json);
}

// ---------------------------------------------------------------------------
// Instance
// ---------------------------------------------------------------------------

#[test]
fn instance_round_trip() {
    let json = r#"{
        "id": "wrld_abc:12345",
        "worldId": "wrld_abc",
        "instanceId": "12345",
        "name": "My Instance",
        "location": "wrld_abc:12345",
        "type": "public",
        "region": "us",
        "capacity": 32,
        "userCount": 10,
        "nUsers": 10,
        "platforms": {"standalonewindows": 8, "android": 2},
        "canRequestInvite": false,
        "full": false
    }"#;
    let inst: Instance = serde_json::from_str(json).unwrap();
    assert_eq!(inst.id, "wrld_abc:12345");
    assert_eq!(inst.user_count, 10);
    assert_eq!(inst.platforms.standalonewindows, Some(8));
}

#[test]
fn instance_platforms_empty() {
    let json = r#"{"standalonewindows": null, "android": null}"#;
    let p: InstancePlatforms = serde_json::from_str(json).unwrap();
    assert!(p.standalonewindows.is_none());
    assert!(p.android.is_none());
}

// ---------------------------------------------------------------------------
// Favorite / FavoriteGroup
// ---------------------------------------------------------------------------

#[test]
fn favorite_round_trip() {
    let json = r#"{
        "id": "fvrt_1",
        "favoriteId": "wrld_abc",
        "type": "world",
        "tags": ["worlds1"]
    }"#;
    round_trip::<Favorite>(json);
}

#[test]
fn favorite_group_round_trip() {
    let json = r#"{
        "id": "favgrp_1",
        "ownerId": "usr_me",
        "ownerDisplayName": "Me",
        "name": "worlds1",
        "displayName": "Favourite Worlds 1",
        "type": "world",
        "visibility": "private",
        "tags": []
    }"#;
    round_trip::<FavoriteGroup>(json);
}
