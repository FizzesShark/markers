# Markers

Markers is an online platform designed to serve as a one-stop-shop for online course discussions, announcements and submissions.
Inspired by me and my classmates' troubles with various other online services, this hopes to offer possibilities to improve upon existing features.
This project is currently in its earliest stages of development.

## Features (eventual)

  * Email-authenticated registration for teachers and students, with separate account types
  * Creation of individual assignments by teachers
  * Ability to open new discussion threads under assignments
  * Submit files directly on assignment pages
  
## Roadmap

- [ ] Create a user registration page with basic features - username, password, type of account
- [ ] Store said users using MongoDB, and hash passwords using SHA-256
- [ ] Add site-wide authentication using cookies
- [ ] Create a "posts" document in MongoDB, with various fields (subtasks needed)
- [ ] Create an "assignments" document in MongoDB (same as above)
- [ ] Give students the ability to create posts, and teachers both posts and assignments
