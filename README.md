![mufbot banner](https://github.com/user-attachments/assets/7eca9013-276b-4d9b-b608-a976be8dce56)

# Introducing, mufbot.

Developed and maintained by [muffonlabs](https://github.com/muffonlabs), mufbot is an automation bot designed to streamline processes within the [mufbeta](https://github.com/muffonlabs/mufbeta) ecosystem. As part of our beta rollout pipeline, mufbot aims to enhance efficiency and productivity.

# Primary Objectives
- Automate key utilities for the muffon development team.
- Interact with our Discord community to perform tasks that might be helpful.

# Vision
mufbot will play a pivotal role in simplifying daily tasks, fostering collaboration, and enriching the user experience within our community.

# Project Details

## Team
<table>
  <tr>
    <th>Member</th>
    <th>Role</th>
  </tr>
  <tr>
    <td>
      <a href="https://github.com/gamersi">
        gamersi
      </a>
    </td>
    <td>Development</td>
  </tr>
  <tr>
    <td>
      <a href="https://github.com/xyloflake">
        xyloflake
      </a>
    </td>
    <td>Project Supervision/Management</td>
  </tr>
</table>

## Tech stack
- Programming Language: Rust 🦀 (nightly)
- Framework: [Poise](https://github.com/serenity-rs/poise)

# Future goals

- **Extend mufbot from discord only to discord + github**, with seamless integration between both.
- **Synchronize issues between discord (forum posts) and github (issues).**
- **Play audio streams right into discord's voice channels by utilizing muffon's api.** Idea is not considered complete and development would require other chores to be completed first. ***Work may start from next year.*** This idea suggests that mufbot can be used in the future as a music bot for all discord servers.

# Setup
To setup the bot:
1. Clone the repository
2. Create a `.env` file in the root of the repository and add the appropriate values.
3. Either use the prebuilt binary with the `./scripts/get_latest_mufbot.sh` script or build the bot yourself.
