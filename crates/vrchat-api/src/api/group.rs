use serde::Serialize;

use crate::client::VRChatClient;
use crate::error::Result;
use crate::models::group::{
    CalendarEvent, Group, GroupAuditLogList, GroupGalleryImage, GroupMember,
    GroupPost, GroupRole,
};

// ---------------------------------------------------------------------------
// Params
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetGroupParams {
    pub include_roles: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchGroupsParams {
    pub n: Option<u32>,
    pub offset: Option<u32>,
    pub query: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateGroupParams {
    pub name: String,
    pub short_code: String,
    pub discriminator: Option<String>,
    pub description: Option<String>,
    pub join_state: Option<String>,
    pub icon_id: Option<String>,
    pub banner_id: Option<String>,
    pub privacy: Option<String>,
    pub role_template: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateGroupParams {
    pub name: Option<String>,
    pub short_code: Option<String>,
    pub description: Option<String>,
    pub join_state: Option<String>,
    pub icon_id: Option<String>,
    pub banner_id: Option<String>,
    pub privacy: Option<String>,
    pub languages: Option<Vec<String>>,
    pub links: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetGroupMembersParams {
    pub n: Option<u32>,
    pub offset: Option<u32>,
    pub sort: Option<String>,
    pub role_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchGroupMembersParams {
    pub query: Option<String>,
    pub n: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateGroupMemberParams {
    pub visibility: Option<String>,
    pub is_subscribed_to_announcements: Option<bool>,
    pub is_subscribed_to_event_announcements: Option<bool>,
    pub manager_notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetGroupPostsParams {
    pub n: Option<u32>,
    pub offset: Option<u32>,
    pub public_only: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateGroupPostParams {
    pub title: String,
    pub text: String,
    pub image_id: Option<String>,
    pub visibility: Option<String>,
    pub role_ids_to_show: Option<Vec<String>>,
    pub send_notification: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateGroupPostParams {
    pub title: Option<String>,
    pub text: Option<String>,
    pub image_id: Option<String>,
    pub visibility: Option<String>,
    pub role_ids_to_show: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetGroupInvitesParams {
    pub n: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetGroupBansParams {
    pub n: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BanGroupMemberParams {
    pub user_id: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetGroupAuditLogsParams {
    pub n: Option<u32>,
    pub offset: Option<u32>,
    pub start_dt: Option<String>,
    pub end_dt: Option<String>,
    pub actor_id: Option<String>,
    pub event_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateGroupRoleParams {
    pub name: String,
    pub description: Option<String>,
    pub is_self_assignable: Option<bool>,
    pub permissions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateGroupRoleParams {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_self_assignable: Option<bool>,
    pub permissions: Option<Vec<String>>,
    pub order: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetGalleryParams {
    pub n: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CalendarParams {
    pub date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateCalendarEventParams {
    pub name: String,
    pub description: Option<String>,
    pub start_dt: String,
    pub end_dt: Option<String>,
    pub frequency: Option<String>,
    pub image_id: Option<String>,
    pub is_public: Option<bool>,
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCalendarEventParams {
    pub name: Option<String>,
    pub description: Option<String>,
    pub start_dt: Option<String>,
    pub end_dt: Option<String>,
    pub frequency: Option<String>,
    pub image_id: Option<String>,
    pub is_public: Option<bool>,
    pub visibility: Option<String>,
}

// ---------------------------------------------------------------------------
// API impl
// ---------------------------------------------------------------------------

impl VRChatClient {
    // ---- Group CRUD ----

    /// GET /groups/{groupId} – Get a group.
    pub async fn get_group(&self, group_id: &str, params: &GetGroupParams) -> Result<Group> {
        self.get(&format!("groups/{group_id}"), Some(params)).await
    }

    /// POST /groups – Create a group.
    pub async fn create_group(&self, params: &CreateGroupParams) -> Result<Group> {
        self.post("groups", Some(params)).await
    }

    /// PUT /groups/{groupId} – Update a group.
    pub async fn update_group(&self, group_id: &str, params: &UpdateGroupParams) -> Result<Group> {
        self.put(&format!("groups/{group_id}"), Some(params)).await
    }

    /// GET /groups/strictsearch – Strict-search groups.
    pub async fn search_groups_strict(&self, params: &SearchGroupsParams) -> Result<Vec<Group>> {
        self.get("groups/strictsearch", Some(params)).await
    }

    /// GET /groups/roleTemplates – List group role templates.
    pub async fn get_group_role_templates(&self) -> Result<serde_json::Value> {
        self.get("groups/roleTemplates", None::<&()>).await
    }

    // ---- Membership ----

    /// POST /groups/{groupId}/join – Join a group.
    pub async fn join_group(&self, group_id: &str) -> Result<GroupMember> {
        self.post::<_, ()>(&format!("groups/{group_id}/join"), None)
            .await
    }

    /// POST /groups/{groupId}/leave – Leave a group.
    pub async fn leave_group(&self, group_id: &str) -> Result<serde_json::Value> {
        self.post::<_, ()>(&format!("groups/{group_id}/leave"), None)
            .await
    }

    /// GET /users/{userId}/groups – List groups for a user.
    pub async fn get_user_groups(&self, user_id: &str) -> Result<Vec<Group>> {
        self.get(&format!("users/{user_id}/groups"), None::<&()>)
            .await
    }

    /// GET /users/{userId}/groups/represented – Get a user's represented group.
    pub async fn get_user_represented_group(&self, user_id: &str) -> Result<Group> {
        self.get(
            &format!("users/{user_id}/groups/represented"),
            None::<&()>,
        )
        .await
    }

    /// PUT /groups/{groupId}/representation – Set current user's represented group.
    pub async fn set_group_representation(&self, group_id: &str) -> Result<serde_json::Value> {
        self.put::<_, ()>(&format!("groups/{group_id}/representation"), None)
            .await
    }

    /// GET /users/{userId}/groups/permissions – Get a user's group permissions.
    pub async fn get_user_group_permissions(
        &self,
        user_id: &str,
    ) -> Result<serde_json::Value> {
        self.get(
            &format!("users/{user_id}/groups/permissions"),
            None::<&()>,
        )
        .await
    }

    /// GET /users/{userId}/groups/{membershipStatus} – Get groups by membership status.
    pub async fn get_groups_by_membership_status(
        &self,
        user_id: &str,
        membership_status: &str,
    ) -> Result<Vec<Group>> {
        self.get(
            &format!("users/{user_id}/groups/{membership_status}"),
            None::<&()>,
        )
        .await
    }

    /// GET /users/{userId}/instances/groups – List group instances for a user.
    pub async fn get_user_group_instances(&self, user_id: &str) -> Result<serde_json::Value> {
        self.get(
            &format!("users/{user_id}/instances/groups"),
            None::<&()>,
        )
        .await
    }

    /// GET /users/{userId}/instances/groups/{groupId} – List instances for a specific group.
    pub async fn get_group_instances(
        &self,
        user_id: &str,
        group_id: &str,
    ) -> Result<serde_json::Value> {
        self.get(
            &format!("users/{user_id}/instances/groups/{group_id}"),
            None::<&()>,
        )
        .await
    }

    // ---- Members ----

    /// GET /groups/{groupId}/members – List members.
    pub async fn get_group_members(
        &self,
        group_id: &str,
        params: &GetGroupMembersParams,
    ) -> Result<Vec<GroupMember>> {
        self.get(&format!("groups/{group_id}/members"), Some(params))
            .await
    }

    /// GET /groups/{groupId}/members/{userId} – Get a specific member.
    pub async fn get_group_member(
        &self,
        group_id: &str,
        user_id: &str,
    ) -> Result<GroupMember> {
        self.get(
            &format!("groups/{group_id}/members/{user_id}"),
            None::<&()>,
        )
        .await
    }

    /// PUT /groups/{groupId}/members/{userId} – Update a group member.
    pub async fn update_group_member(
        &self,
        group_id: &str,
        user_id: &str,
        params: &UpdateGroupMemberParams,
    ) -> Result<GroupMember> {
        self.put(
            &format!("groups/{group_id}/members/{user_id}"),
            Some(params),
        )
        .await
    }

    /// DELETE /groups/{groupId}/members/{userId} – Kick / unblock a member.
    pub async fn kick_group_member(
        &self,
        group_id: &str,
        user_id: &str,
    ) -> Result<serde_json::Value> {
        self.delete(
            &format!("groups/{group_id}/members/{user_id}"),
            None::<&()>,
        )
        .await
    }

    /// GET /groups/{groupId}/members/search – Search members.
    pub async fn search_group_members(
        &self,
        group_id: &str,
        params: &SearchGroupMembersParams,
    ) -> Result<Vec<GroupMember>> {
        self.get(
            &format!("groups/{group_id}/members/search"),
            Some(params),
        )
        .await
    }

    // ---- Member roles ----

    /// PUT /groups/{groupId}/members/{userId}/roles/{roleId} – Add role to member.
    pub async fn add_group_member_role(
        &self,
        group_id: &str,
        user_id: &str,
        role_id: &str,
    ) -> Result<serde_json::Value> {
        self.put::<_, ()>(
            &format!("groups/{group_id}/members/{user_id}/roles/{role_id}"),
            None,
        )
        .await
    }

    /// DELETE /groups/{groupId}/members/{userId}/roles/{roleId} – Remove role from member.
    pub async fn remove_group_member_role(
        &self,
        group_id: &str,
        user_id: &str,
        role_id: &str,
    ) -> Result<serde_json::Value> {
        self.delete(
            &format!("groups/{group_id}/members/{user_id}/roles/{role_id}"),
            None::<&()>,
        )
        .await
    }

    // ---- Roles ----

    /// GET /groups/{groupId}/roles – List roles.
    pub async fn get_group_roles(&self, group_id: &str) -> Result<Vec<GroupRole>> {
        self.get(&format!("groups/{group_id}/roles"), None::<&()>)
            .await
    }

    /// POST /groups/{groupId}/roles – Create a role.
    pub async fn create_group_role(
        &self,
        group_id: &str,
        params: &CreateGroupRoleParams,
    ) -> Result<GroupRole> {
        self.post(&format!("groups/{group_id}/roles"), Some(params))
            .await
    }

    /// PUT /groups/{groupId}/roles/{roleId} – Update a role.
    pub async fn update_group_role(
        &self,
        group_id: &str,
        role_id: &str,
        params: &UpdateGroupRoleParams,
    ) -> Result<GroupRole> {
        self.put(
            &format!("groups/{group_id}/roles/{role_id}"),
            Some(params),
        )
        .await
    }

    // ---- Invites ----

    /// GET /groups/{groupId}/invites – List pending invites.
    pub async fn get_group_invites(
        &self,
        group_id: &str,
        params: &GetGroupInvitesParams,
    ) -> Result<Vec<GroupMember>> {
        self.get(&format!("groups/{group_id}/invites"), Some(params))
            .await
    }

    /// POST /groups/{groupId}/invites – Invite a user.
    pub async fn invite_to_group(
        &self,
        group_id: &str,
        user_id: &str,
    ) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Body<'a> {
            user_id: &'a str,
        }
        self.post(
            &format!("groups/{group_id}/invites"),
            Some(&Body { user_id }),
        )
        .await
    }

    /// DELETE /groups/{groupId}/invites/{userId} – Cancel an invite.
    pub async fn cancel_group_invite(
        &self,
        group_id: &str,
        user_id: &str,
    ) -> Result<serde_json::Value> {
        self.delete(
            &format!("groups/{group_id}/invites/{user_id}"),
            None::<&()>,
        )
        .await
    }

    // ---- Join requests ----

    /// GET /groups/{groupId}/requests – List join requests.
    pub async fn get_group_requests(
        &self,
        group_id: &str,
        params: &GetGroupInvitesParams,
    ) -> Result<Vec<GroupMember>> {
        self.get(&format!("groups/{group_id}/requests"), Some(params))
            .await
    }

    /// PUT /groups/{groupId}/requests/{userId} – Approve/deny a join request.
    pub async fn respond_group_request(
        &self,
        group_id: &str,
        user_id: &str,
        action: &str,
    ) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Body<'a> {
            action: &'a str,
        }
        self.put(
            &format!("groups/{group_id}/requests/{user_id}"),
            Some(&Body { action }),
        )
        .await
    }

    /// DELETE /groups/{groupId}/requests/{userId} – Delete a join request.
    pub async fn delete_group_request(
        &self,
        group_id: &str,
        user_id: &str,
    ) -> Result<serde_json::Value> {
        self.delete(
            &format!("groups/{group_id}/requests/{user_id}"),
            None::<&()>,
        )
        .await
    }

    // ---- Bans ----

    /// GET /groups/{groupId}/bans – List bans.
    pub async fn get_group_bans(
        &self,
        group_id: &str,
        params: &GetGroupBansParams,
    ) -> Result<Vec<GroupMember>> {
        self.get(&format!("groups/{group_id}/bans"), Some(params))
            .await
    }

    /// POST /groups/{groupId}/bans – Ban a user.
    pub async fn ban_group_member(
        &self,
        group_id: &str,
        params: &BanGroupMemberParams,
    ) -> Result<GroupMember> {
        self.post(&format!("groups/{group_id}/bans"), Some(params))
            .await
    }

    /// DELETE /groups/{groupId}/bans/{userId} – Unban a user.
    pub async fn unban_group_member(
        &self,
        group_id: &str,
        user_id: &str,
    ) -> Result<GroupMember> {
        self.delete(
            &format!("groups/{group_id}/bans/{user_id}"),
            None::<&()>,
        )
        .await
    }

    // ---- Block / Unblock group ----

    /// POST /groups/{groupId}/block – Block a group.
    pub async fn block_group(&self, group_id: &str) -> Result<serde_json::Value> {
        self.post::<_, ()>(&format!("groups/{group_id}/block"), None)
            .await
    }

    // ---- Posts ----

    /// GET /groups/{groupId}/posts – List posts.
    pub async fn get_group_posts(
        &self,
        group_id: &str,
        params: &GetGroupPostsParams,
    ) -> Result<Vec<GroupPost>> {
        self.get(&format!("groups/{group_id}/posts"), Some(params))
            .await
    }

    /// POST /groups/{groupId}/posts – Create a post.
    pub async fn create_group_post(
        &self,
        group_id: &str,
        params: &CreateGroupPostParams,
    ) -> Result<GroupPost> {
        self.post(&format!("groups/{group_id}/posts"), Some(params))
            .await
    }

    /// PUT /groups/{groupId}/posts/{postId} – Update a post.
    pub async fn update_group_post(
        &self,
        group_id: &str,
        post_id: &str,
        params: &UpdateGroupPostParams,
    ) -> Result<GroupPost> {
        self.put(
            &format!("groups/{group_id}/posts/{post_id}"),
            Some(params),
        )
        .await
    }

    /// DELETE /groups/{groupId}/posts/{postId} – Delete a post.
    pub async fn delete_group_post(
        &self,
        group_id: &str,
        post_id: &str,
    ) -> Result<serde_json::Value> {
        self.delete(
            &format!("groups/{group_id}/posts/{post_id}"),
            None::<&()>,
        )
        .await
    }

    // ---- Audit logs ----

    /// GET /groups/{groupId}/auditLogTypes – List audit log types.
    pub async fn get_group_audit_log_types(
        &self,
        group_id: &str,
    ) -> Result<serde_json::Value> {
        self.get(
            &format!("groups/{group_id}/auditLogTypes"),
            None::<&()>,
        )
        .await
    }

    /// GET /groups/{groupId}/auditLogs – Get audit logs.
    pub async fn get_group_audit_logs(
        &self,
        group_id: &str,
        params: &GetGroupAuditLogsParams,
    ) -> Result<GroupAuditLogList> {
        self.get(&format!("groups/{group_id}/auditLogs"), Some(params))
            .await
    }

    // ---- Gallery ----

    /// GET /groups/{groupId}/galleries/{galleryId} – Get gallery images.
    pub async fn get_group_gallery(
        &self,
        group_id: &str,
        gallery_id: &str,
        params: &GetGalleryParams,
    ) -> Result<Vec<GroupGalleryImage>> {
        self.get(
            &format!("groups/{group_id}/galleries/{gallery_id}"),
            Some(params),
        )
        .await
    }

    // ---- Calendar / Events ----

    /// GET /calendar/{groupId} – Get a group calendar.
    pub async fn get_group_calendar(
        &self,
        group_id: &str,
        params: &CalendarParams,
    ) -> Result<Vec<CalendarEvent>> {
        self.get(&format!("calendar/{group_id}"), Some(params))
            .await
    }

    /// GET /calendar – Get all calendars.
    pub async fn get_calendar(&self, params: &CalendarParams) -> Result<Vec<CalendarEvent>> {
        self.get("calendar", Some(params)).await
    }

    /// GET /calendar/following – Get calendars for followed groups.
    pub async fn get_following_calendar(
        &self,
        params: &CalendarParams,
    ) -> Result<Vec<CalendarEvent>> {
        self.get("calendar/following", Some(params)).await
    }

    /// GET /calendar/featured – Get featured events.
    pub async fn get_featured_calendar(
        &self,
        params: &CalendarParams,
    ) -> Result<Vec<CalendarEvent>> {
        self.get("calendar/featured", Some(params)).await
    }

    /// GET /calendar/{groupId}/{eventId} – Get a specific event.
    pub async fn get_calendar_event(
        &self,
        group_id: &str,
        event_id: &str,
    ) -> Result<CalendarEvent> {
        self.get(
            &format!("calendar/{group_id}/{event_id}"),
            None::<&()>,
        )
        .await
    }

    /// POST /calendar/{groupId}/event – Create an event.
    pub async fn create_calendar_event(
        &self,
        group_id: &str,
        params: &CreateCalendarEventParams,
    ) -> Result<CalendarEvent> {
        self.post(&format!("calendar/{group_id}/event"), Some(params))
            .await
    }

    /// PUT /calendar/{groupId}/{eventId} – Update an event.
    pub async fn update_calendar_event(
        &self,
        group_id: &str,
        event_id: &str,
        params: &UpdateCalendarEventParams,
    ) -> Result<CalendarEvent> {
        self.put(
            &format!("calendar/{group_id}/{event_id}"),
            Some(params),
        )
        .await
    }

    /// DELETE /calendar/{groupId}/{eventId} – Delete an event.
    pub async fn delete_calendar_event(
        &self,
        group_id: &str,
        event_id: &str,
    ) -> Result<serde_json::Value> {
        self.delete(
            &format!("calendar/{group_id}/{event_id}"),
            None::<&()>,
        )
        .await
    }

    /// POST /calendar/{groupId}/{eventId}/follow – Follow an event.
    pub async fn follow_calendar_event(
        &self,
        group_id: &str,
        event_id: &str,
    ) -> Result<serde_json::Value> {
        self.post::<_, ()>(
            &format!("calendar/{group_id}/{event_id}/follow"),
            None,
        )
        .await
    }

    /// DELETE /calendar/{groupId}/{eventId}/follow – Unfollow an event.
    pub async fn unfollow_calendar_event(
        &self,
        group_id: &str,
        event_id: &str,
    ) -> Result<serde_json::Value> {
        self.delete(
            &format!("calendar/{group_id}/{event_id}/follow"),
            None::<&()>,
        )
        .await
    }
}
