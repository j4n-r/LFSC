# LFSC

A chat app for university.

## Architecture

The project consists of three main components:

- **sc-core** - Rust-based WebSocket server handling real-time messaging
- **sc-admin** - Flask web application providing the administrative interface
- **sc-mobile** - React Native mobile application for student access

## Features

### For Professors (Administrators)
- Create, reorder, and delete student groups instantly
- Monitor all student communications transparently
- Complete administrative oversight and user management
- Prevent misuse and maintain academic integrity

### For Students
- Access to group chats and communication features
- Seamless and organized academic discussions
- Mobile-friendly interface for on-the-go communication

# Running the project
For now check out the READMEs for `sc-mobile/README.md` and `sc-admin/README.md`


## Project Structure

```
LFSC/
├── sc-admin/          # Flask web application
│   ├── app/           # Application modules
│   ├── docs/          # Project documentation
│   └── instance/      # Database and configuration
├── sc-core/           # Rust WebSocket server
│   └── src/           # Server source code
├── sc-mobile/         # React Native mobile app
│   ├── app/           # App screens and navigation
│   └── components/    # Reusable UI components
└── scripts/           # Utility scripts
```

## Technology Stack

- **Backend**: Rust (Tokio, SQLx, WebSockets)
- **Web Frontend**: Python Flask, TailwindCSS
- **Mobile**: React Native, Expo
- **Database**: SQLite
- **Development**: Nix flakes for reproducible environments
