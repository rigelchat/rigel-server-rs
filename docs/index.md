<div align="center">

  # 📄 Rigel API Documentation (old)
</div>

# 1. Instance

## ![GET](https://img.shields.io/badge/GET-61affe?style=flat-square) Get Instance Info

`/ping`

Returns information about the Rigel instance.

<details>
<summary><b>Response</b></summary>

```json
{
  "ping": "pong!",
  "instance": {
    "name": "My Rigel Instance",
    "description": "A self-hosted Rigel server",
    "image": "https://example.com/logo.png",
    "correspondenceEmail": "admin@example.com",
    "correspondenceUserID": "123456789012345678",
    "frontPage": "https://example.com",
    "tosPage": "https://example.com/tos",
    "authMethods": {
      "discord": {
        "clientId": "123456789012345678",
        "authUrl": "https://example.com/api/v0/auth/discord-callback"
      }
    }
  }
}
```

</details>

---

## ![GET](https://img.shields.io/badge/GET-61affe?style=flat-square) Get Instance Domains

`/policies/instance/domains`

Returns domain configuration for the instance.

<details>
<summary><b>Response</b></summary>

```json
{
  "cdn": "https://example.com/_cdn",
  "apiEndpoint": "https://example.com/api",
  "defaultApiVersion": 0,
  "compress": "snappy"
}
```

</details>

---


# 2. Authentication

## ![GET](https://img.shields.io/badge/GET-61affe?style=flat-square) Discord Callback

`/auth/discord-callback`

Authenticates a user via Discord OAuth2 and returns a session token.

<details>
<summary><b>Query Parameters</b></summary>

| Parameter      | Type     | Required | Description                    |
| -------------- | -------- | :------: | ------------------------------ |
| `code`         | `string` |    ✅    | OAuth2 authorization code      |
| `redirect_uri` | `string` |    ✅    | OAuth2 redirect URI (must be valid URL) |

</details>

<details>
<summary><b>Response</b></summary>

```json
{
  "token": "MTIzNDU2Nzg5MDEyMzQ1Njc4OQ.XXXXXX.XXXXXXXXXXXXXXXXXXXXXXXX"
}
```

</details>

---

## ![GET](https://img.shields.io/badge/GET-61affe?style=flat-square) Get Auth Sessions

`/auth/sessions`

Returns all active sessions for the authenticated user.

<details>
<summary><b>Headers</b></summary>

| Header          | Type     | Required | Description      |
| --------------- | -------- | :------: | ---------------- |
| `Authorization` | `string` |    ✅    | User token       |

</details>

<details>
<summary><b>Response</b></summary>

```json
{
  "user_sessions": [
    {
      "id_hash": "abc123",
      "approx_last_used_time": 1701878400000,
      "client_info": {
        "os": "Windows",
        "platform": "Desktop",
        "location": "France"
      }
    }
  ]
}
```

</details>

---

## ![POST](https://img.shields.io/badge/POST-49cc90?style=flat-square) Logout Auth Sessions

`/auth/sessions/logout`

Logs out one or more sessions by their ID hashes.

<details>
<summary><b>Headers</b></summary>

| Header          | Type     | Required | Description      |
| --------------- | -------- | :------: | ---------------- |
| `Authorization` | `string` |    ✅    | User token       |

</details>

<details>
<summary><b>Request Body</b></summary>

```json
{
  "session_id_hashes": ["abc123", "def456"]
}
```

| Field               | Type       | Required | Description                          |
| ------------------- | ---------- | :------: | ------------------------------------ |
| `session_id_hashes` | `string[]` |    ✅    | Array of session IDs to logout (1-100) |

</details>

<details>
<summary><b>Response</b></summary>

`204 No Content`

</details>

---

# 3. Gateway

## ![GET](https://img.shields.io/badge/GET-61affe?style=flat-square) Get Gateway

`/gateway`

Returns the WebSocket URL for connecting to the gateway.

<details>
<summary><b>Response</b></summary>

```json
{
  "url": "wss://gateway.example.com/gateway"
}
```

</details>

---

## ![GET](https://img.shields.io/badge/GET-61affe?style=flat-square) Get Gateway Bot

`/gateway/bot`

Returns the WebSocket URL and sharding information for bots.

<details>
<summary><b>Response</b></summary>

```json
{
  "url": "wss://gateway.example.com/gateway",
  "shards": 1,
  "session_start_limit": {
    "remaining": 0,
    "total": 1,
    "max_concurrency": 1,
    "reset_after": 14400000
  }
}
```

</details>

---


# 4. Discovery

## ![GET](https://img.shields.io/badge/GET-61affe?style=flat-square) Get Discoverable Guilds

`/discoverable-guilds`

Returns a list of all discoverable guilds.

<details>
<summary><b>Headers</b></summary>

| Header          | Type     | Required | Description      |
| --------------- | -------- | :------: | ---------------- |
| `Authorization` | `string` |    ✅    | User token       |

</details>

<details>
<summary><b>Response</b></summary>

```json
{
  "guilds": [
    {
      "id": "123456789012345678",
      "name": "My Server",
      "icon": "a_1234567890abcdef",
      "banner": null,
      "description": "A cool server",
      "vanity_url_code": "myserver",
      "approximate_member_count": 150,
      "approximate_presence_count": 42
    }
  ],
  "offset": 0,
  "limit": 10,
  "total": 1
}
```

</details>

---


# 5. Guilds

## ![POST](https://img.shields.io/badge/POST-49cc90?style=flat-square) Create Guild

`/guilds`

Creates a new guild. Requires `Staff` flag.

<details>
<summary><b>Request Body</b></summary>

```json
{
  "name": "My New Server",
  "icon": "data:image/png;base64,..."
}
```

| Field  | Type      | Required | Description                        |
| ------ | --------- | :------: | ---------------------------------- |
| `name` | `string`  |    ✅    | Guild name (2-100 characters)      |
| `icon` | `string?` |    ❌    | Base64 encoded image (png/jpg/gif/webp) |

</details>

<details>
<summary><b>Response</b></summary>

Returns a complete [Guild](#guild-object) object with channels, roles, and members.

</details>

---

## ![PATCH](https://img.shields.io/badge/PATCH-50e3c2?style=flat-square) Modify Guild

`/guilds/{guild.id}`

Modifies a guild's settings. Requires `MANAGE_GUILD` permission.

<details>
<summary><b>Path Parameters</b></summary>

| Parameter  | Type        | Description |
| ---------- | ----------- | ----------- |
| `guild.id` | `snowflake` | Guild ID    |

</details>

<details>
<summary><b>Request Body</b></summary>

```json
{
  "name": "New Name",
  "icon": "data:image/png;base64,...",
  "banner": null,
  "description": "A cool server description",
  "system_channel_id": "123456789012345678",
  "rules_channel_id": null
}
```

| Field               | Type         | Required | Description                        |
| ------------------- | ------------ | :------: | ---------------------------------- |
| `name`              | `string`     |    ❌    | Guild name (2-100 characters)      |
| `icon`              | `string?`    |    ❌    | Base64 image or `null` to remove   |
| `banner`            | `string?`    |    ❌    | Base64 image or `null` to remove   |
| `description`       | `string?`    |    ❌    | Guild description (max 1000 chars) |
| `system_channel_id` | `snowflake?` |    ❌    | System messages channel ID         |
| `rules_channel_id`  | `snowflake?` |    ❌    | Rules channel ID                   |

</details>

---

## ![POST](https://img.shields.io/badge/POST-49cc90?style=flat-square) Delete Guild

`/guilds/{guild.id}/delete`

Permanently deletes a guild. Must be the guild owner.

<details>
<summary><b>Path Parameters</b></summary>

| Parameter  | Type        | Description |
| ---------- | ----------- | ----------- |
| `guild.id` | `snowflake` | Guild ID    |

</details>

<details>
<summary><b>Response</b></summary>

`204 No Content`

</details>

---

## ![DELETE](https://img.shields.io/badge/DELETE-f93e3e?style=flat-square) Kick Guild Member

`/guilds/{guild.id}/members/{user.id}`

Kicks a member from the guild. Requires `KICK_MEMBERS` permission.

<details>
<summary><b>Path Parameters</b></summary>

| Parameter  | Type        | Description      |
| ---------- | ----------- | ---------------- |
| `guild.id` | `snowflake` | Guild ID         |
| `user.id`  | `snowflake` | User ID to kick  |

</details>

<details>
<summary><b>Response</b></summary>

`204 No Content`

</details>

---

## ![PUT](https://img.shields.io/badge/PUT-fca130?style=flat-square) Add Guild Member Role

`/guilds/{guild.id}/members/{user.id}/roles/{role.id}`

Adds a role to a guild member. Requires `MANAGE_ROLES` permission.

<details>
<summary><b>Path Parameters</b></summary>

| Parameter  | Type        | Description |
| ---------- | ----------- | ----------- |
| `guild.id` | `snowflake` | Guild ID    |
| `user.id`  | `snowflake` | User ID     |
| `role.id`  | `snowflake` | Role ID     |

</details>

<details>
<summary><b>Response</b></summary>

`204 No Content`

</details>

---

## ![DELETE](https://img.shields.io/badge/DELETE-f93e3e?style=flat-square) Remove Guild Member Role

`/guilds/{guild.id}/members/{user.id}/roles/{role.id}`

Removes a role from a guild member. Requires `MANAGE_ROLES` permission.

<details>
<summary><b>Path Parameters</b></summary>

| Parameter  | Type        | Description |
| ---------- | ----------- | ----------- |
| `guild.id` | `snowflake` | Guild ID    |
| `user.id`  | `snowflake` | User ID     |
| `role.id`  | `snowflake` | Role ID     |

</details>

<details>
<summary><b>Response</b></summary>

`204 No Content`

</details>

---

## ![POST](https://img.shields.io/badge/POST-49cc90?style=flat-square) Create Guild Role

`/guilds/{guild.id}/roles`

Creates a new role in the guild. Requires `MANAGE_ROLES` permission.

<details>
<summary><b>Path Parameters</b></summary>

| Parameter  | Type        | Description |
| ---------- | ----------- | ----------- |
| `guild.id` | `snowflake` | Guild ID    |

</details>

<details>
<summary><b>Request Body</b></summary>

```json
{
  "name": "Moderators",
  "color": 3447003,
  "hoist": true,
  "unicode_emoji": "🛡️",
  "permissions": "1099511627775",
  "mentionable": true
}
```

| Field           | Type      | Required | Description                    |
| --------------- | --------- | :------: | ------------------------------ |
| `name`          | `string`  |    ❌    | Role name (default: "new role") |
| `color`         | `integer` |    ❌    | RGB color value                |
| `hoist`         | `boolean` |    ❌    | Display separately in sidebar  |
| `unicode_emoji` | `string?` |    ❌    | Unicode emoji for role icon    |
| `permissions`   | `string`  |    ❌    | Permission bit set as string   |
| `mentionable`   | `boolean` |    ❌    | Allow anyone to @mention role  |

</details>

---

## ![PATCH](https://img.shields.io/badge/PATCH-50e3c2?style=flat-square) Modify Guild Role Positions

`/guilds/{guild.id}/roles`

Modifies the positions of roles in the guild. Requires `MANAGE_ROLES` permission.

<details>
<summary><b>Path Parameters</b></summary>

| Parameter  | Type        | Description |
| ---------- | ----------- | ----------- |
| `guild.id` | `snowflake` | Guild ID    |

</details>

<details>
<summary><b>Request Body</b></summary>

```json
[
  { "id": "123456789012345678", "position": 1 },
  { "id": "234567890123456789", "position": 2 }
]
```

</details>

---

## ![PATCH](https://img.shields.io/badge/PATCH-50e3c2?style=flat-square) Modify Guild Role

`/guilds/{guild.id}/roles/{role.id}`

Modifies a role's settings. Requires `MANAGE_ROLES` permission.

<details>
<summary><b>Path Parameters</b></summary>

| Parameter  | Type        | Description |
| ---------- | ----------- | ----------- |
| `guild.id` | `snowflake` | Guild ID    |
| `role.id`  | `snowflake` | Role ID     |

</details>

<details>
<summary><b>Request Body</b></summary>

Same as [Create Guild Role](#-create-guild-role).

</details>

---

# 6. Invites

## ![POST](https://img.shields.io/badge/POST-49cc90?style=flat-square) Accept Invite

`/invites/{invite.code}`

Joins a guild using its vanity URL code.

<details>
<summary><b>Path Parameters</b></summary>

| Parameter     | Type     | Description              |
| ------------- | -------- | ------------------------ |
| `invite.code` | `string` | Vanity URL code (1-32 chars) |

</details>

<details>
<summary><b>Request Body</b></summary>

```json
{
  "session_id": "abc123def456"
}
```

| Field        | Type     | Required | Description      |
| ------------ | -------- | :------: | ---------------- |
| `session_id` | `string` |    ❌    | Gateway session ID |

</details>

<details>
<summary><b>Response</b></summary>

```json
{
  "id": 0,
  "type": 0,
  "code": "myserver",
  "expires_at": null,
  "flags": 0,
  "guild_id": "123456789012345678",
  "guild": null,
  "channel": null,
  "new_member": true
}
```

</details>

---

# 7. Messages

## ![GET](https://img.shields.io/badge/GET-61affe?style=flat-square) Get Channel Messages

`/channels/{channel.id}/messages`

Returns messages from a channel. Requires `VIEW_CHANNEL` and `READ_MESSAGE_HISTORY` permissions.

<details>
<summary><b>Path Parameters</b></summary>

| Parameter    | Type        | Description |
| ------------ | ----------- | ----------- |
| `channel.id` | `snowflake` | Channel ID  |

</details>

<details>
<summary><b>Query Parameters</b></summary>

| Parameter | Type        | Required | Description                         |
| --------- | ----------- | :------: | ----------------------------------- |
| `limit`   | `integer`   |    ❌    | Number of messages (1-100, default: 50) |
| `before`  | `snowflake` |    ❌    | Get messages before this ID         |
| `after`   | `snowflake` |    ❌    | Get messages after this ID          |
| `around`  | `snowflake` |    ❌    | Get messages around this ID         |

</details>

<details>
<summary><b>Response</b></summary>

```json
[
  {
    "id": "123456789012345678",
    "type": 0,
    "timestamp": "2024-12-06T12:00:00.000Z",
    "edited_timestamp": null,
    "channel_id": "234567890123456789",
    "content": "Hello, world!",
    "flags": 0,
    "author": {
      "id": "345678901234567890",
      "username": "User",
      "global_name": "Display Name",
      "avatar": "a_1234567890abcdef",
      "banner": null,
      "public_flags": 0
    }
  }
]
```

</details>

---

## ![POST](https://img.shields.io/badge/POST-49cc90?style=flat-square) Create Message

`/channels/{channel.id}/messages`

Sends a message to a channel. Requires `VIEW_CHANNEL` and `SEND_MESSAGES` permissions.

<details>
<summary><b>Path Parameters</b></summary>

| Parameter    | Type        | Description |
| ------------ | ----------- | ----------- |
| `channel.id` | `snowflake` | Channel ID  |

</details>

<details>
<summary><b>Request Body</b></summary>

```json
{
  "content": "Hello, world!",
  "nonce": "unique-id-123",
  "tts": false
}
```

| Field     | Type      | Required | Description                     |
| --------- | --------- | :------: | ------------------------------- |
| `content` | `string`  |    ✅    | Message content (1-4000 chars)  |
| `nonce`   | `string`  |    ❌    | Nonce for message deduplication |
| `tts`     | `boolean` |    ❌    | Text-to-speech (default: false) |

</details>

<details>
<summary><b>Response</b></summary>

Returns a [Message](#message-object) object.

</details>

---

## ![PATCH](https://img.shields.io/badge/PATCH-50e3c2?style=flat-square) Edit Message

`/channels/{channel.id}/messages/{message.id}`

Edits a previously sent message. Only the author can edit their messages.

<details>
<summary><b>Path Parameters</b></summary>

| Parameter    | Type        | Description |
| ------------ | ----------- | ----------- |
| `channel.id` | `snowflake` | Channel ID  |
| `message.id` | `snowflake` | Message ID  |

</details>

<details>
<summary><b>Request Body</b></summary>

```json
{
  "content": "Edited message content"
}
```

| Field     | Type     | Required | Description                    |
| --------- | -------- | :------: | ------------------------------ |
| `content` | `string` |    ✅    | New message content (1-4000 chars) |

</details>

---

## ![DELETE](https://img.shields.io/badge/DELETE-f93e3e?style=flat-square) Delete Message

`/channels/{channel.id}/messages/{message.id}`

Deletes a message. Authors can delete their own messages, or requires `MANAGE_MESSAGES` permission.

<details>
<summary><b>Path Parameters</b></summary>

| Parameter    | Type        | Description |
| ------------ | ----------- | ----------- |
| `channel.id` | `snowflake` | Channel ID  |
| `message.id` | `snowflake` | Message ID  |

</details>

<details>
<summary><b>Response</b></summary>

`204 No Content`

</details>

---

# 8. Users

## ![DELETE](https://img.shields.io/badge/DELETE-f93e3e?style=flat-square) Leave Guild

`/users/@me/guilds/{guild.id}`

Leaves a guild. Cannot leave if you are the owner.

<details>
<summary><b>Path Parameters</b></summary>

| Parameter  | Type        | Description |
| ---------- | ----------- | ----------- |
| `guild.id` | `snowflake` | Guild ID    |

</details>

<details>
<summary><b>Response</b></summary>

`204 No Content`

</details>

---

## ![PATCH](https://img.shields.io/badge/PATCH-50e3c2?style=flat-square) Modify User Settings

`/users/@me/settings`

Updates the authenticated user's settings.

<details>
<summary><b>Request Body</b></summary>

```json
{
  "status": "online",
  "locale": "fr",
  "theme": "dark",
  "developer_mode": true,
  "background_gradient_preset": "midnight-blurple"
}
```

| Field                       | Type      | Required | Description                           |
| --------------------------- | --------- | :------: | ------------------------------------- |
| `status`                    | `string`  |    ❌    | `online`, `idle`, `dnd`, `invisible`  |
| `locale`                    | `string`  |    ❌    | User locale (2-6 chars)               |
| `theme`                     | `string`  |    ❌    | `dark` or `light`                     |
| `developer_mode`            | `boolean` |    ❌    | Enable developer mode                 |
| `background_gradient_preset`| `string?` |    ❌    | Background gradient preset name       |

</details>

<details>
<summary><b>Available Gradient Presets</b></summary>

`mint-apple` · `citrus-sherbert` · `retro-raincloud` · `hanami` · `sunrise` · `cotton-candy` · `lofi-vibes` · `desert-khaki` · `sunset` · `chroma-glow` · `forest` · `crimson-moon` · `midnight-blurple` · `mars` · `dusk` · `under-the-sea` · `retro-storm` · `neon-nights` · `strawberry-lemonade` · `aurora` · `sepia` · `blurple-twilight`

</details>

---

## ![GET](https://img.shields.io/badge/GET-61affe?style=flat-square) Get User Profile

`/users/{user.id}/profile`

Returns a user's profile information.

<details>
<summary><b>Path Parameters</b></summary>

| Parameter | Type        | Description |
| --------- | ----------- | ----------- |
| `user.id` | `snowflake` | User ID     |

</details>

<details>
<summary><b>Response</b></summary>

```json
{
  "user_profile": {
    "bio": "Hello, I'm a cool user!",
    "pronouns": "they/them",
    "accent_color": 16711680,
    "theme_colors": [16711680, 255]
  }
}
```

</details>

---

# 9. Bots

> ⚠️ **Staff Only** - These endpoints require the `Staff` user flag.

## ![POST](https://img.shields.io/badge/POST-49cc90?style=flat-square) Create Bot

`/bots`

Creates a new bot account.

<details>
<summary><b>Request Body</b></summary>

```json
{
  "username": "MyBot",
  "avatar": "data:image/png;base64,...",
  "banner": null
}
```

| Field      | Type      | Required | Description                      |
| ---------- | --------- | :------: | -------------------------------- |
| `username` | `string`  |    ✅    | Bot username (2-32 characters)   |
| `avatar`   | `string?` |    ❌    | Base64 encoded avatar image      |
| `banner`   | `string?` |    ❌    | Base64 encoded banner image      |

</details>

<details>
<summary><b>Response</b></summary>

```json
{
  "id": "123456789012345678",
  "username": "MyBot",
  "discriminator": "0",
  "avatar": null,
  "banner": null,
  "bot": true,
  "public_flags": 0,
  "token": "MTIzNDU2Nzg5MDEyMzQ1Njc4OQ.XXXXXX.XXXXXXXXXXXXXXXXXXXXXXXX"
}
```

</details>

---

## ![POST](https://img.shields.io/badge/POST-49cc90?style=flat-square) Reset Bot Token

`/bots/{bot.id}/reset`

Regenerates the token for a bot.

<details>
<summary><b>Path Parameters</b></summary>

| Parameter | Type        | Description |
| --------- | ----------- | ----------- |
| `bot.id`  | `snowflake` | Bot ID      |

</details>

<details>
<summary><b>Response</b></summary>

```json
{
  "token": "MTIzNDU2Nzg5MDEyMzQ1Njc4OQ.YYYYYY.YYYYYYYYYYYYYYYYYYYYYYYY"
}
```

</details>

---

## ![GET](https://img.shields.io/badge/GET-61affe?style=flat-square) Get Bot

`/bots/{bot.id}`

Returns information about a bot.

<details>
<summary><b>Path Parameters</b></summary>

| Parameter | Type        | Description |
| --------- | ----------- | ----------- |
| `bot.id`  | `snowflake` | Bot ID      |

</details>

<details>
<summary><b>Response</b></summary>

```json
{
  "id": "123456789012345678",
  "username": "MyBot",
  "discriminator": "0",
  "avatar": null,
  "banner": null,
  "bot": true,
  "public_flags": 0
}
```

</details>

---

## ![DELETE](https://img.shields.io/badge/DELETE-f93e3e?style=flat-square) Delete Bot

`/bots/{bot.id}`

Permanently deletes a bot account.

<details>
<summary><b>Path Parameters</b></summary>

| Parameter | Type        | Description |
| --------- | ----------- | ----------- |
| `bot.id`  | `snowflake` | Bot ID      |

</details>

<details>
<summary><b>Response</b></summary>

`204 No Content`

</details>

---

## ![PUT](https://img.shields.io/badge/PUT-fca130?style=flat-square) Add Bot to Guild

`/bots/{bot.id}/guilds/{guild.id}`

Adds a bot to a guild.

<details>
<summary><b>Path Parameters</b></summary>

| Parameter  | Type        | Description |
| ---------- | ----------- | ----------- |
| `bot.id`   | `snowflake` | Bot ID      |
| `guild.id` | `snowflake` | Guild ID    |

</details>

<details>
<summary><b>Response</b></summary>

`204 No Content`

</details>

---

<div align="center">

Made with ❤️ by the [Rigel Team](https://github.com/rigelchat)

</div>