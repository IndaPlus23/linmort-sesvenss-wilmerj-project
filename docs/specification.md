# Specification

## Description

### Feasability

## Branches

### Main

The main branch will contain the most recent stable and production-ready version of the project. This branch is to always be protected and merges are expected to be thoroughly tested and verified.

### Development

The development branch is where new features and ongoing work are integrated and tested. Versions of this branch are necessarily not always stable nor working as intended. Once a version has met the criteria for a stable release it will be merged into the main branch.

## Naming convention

### Issues

Issues will be created for every feature or task that is needed. The title should summarize the essence and quickly convey the nature of the issue. if required, a detailed description with further information may be included. To help further categorize each issue will be assigned labels based on attributes like type or priority. Assignees will be assigned and take responsibility for the issue. Issues with deadlines will be added under milestones. Examples of good issue titles are `Implement asset pipeline for importing custom textures` with the label `Feature` or `Player movement is jittery near walls` labeled as `Bug`.

> Issues with small or insignificant changes that do not require their own issue may be committed directly to its according branch.

### Commits

Commit messages should provide a concise but informative explanation of the change. In accordance with Git standards commits will be written in an imperative mood starting with a verb indicating the action to be performed. Ideally, the message should fit within a single sentence. However, if the message alone is insufficient in conveying the change or if the situation demands so a shorter description is to be included. Examples of good commit messages are `Update specification.md` or `Fix collision detection with walls`, where the latter may require an additional description.

### Pull requests

## MoSCoW

The MoSCoW method will be used to prioritize requirements in categories of Must have, Should have, Could have, and Won't have.

### Must have

* Rendering logic closely resembling the original 1993 Doom game.
* Basic structures like walls, floors, ceilings and doors.
* Working collision and noclip toggle.
* Sprites that always face the player.
* Textures.

### Should have

* A real time map editor to easily add, edit and remove level contents.
* Binary space partitioning as famously popularized by John Carmack.
* Basic FPS mechanics like shooting, reloading and displaying the ammunition.
* Audio and music system.

### Could have

* Hardware acceleration using a shader language like GLSL.
* Baked in ray traced light maps.
* Portals allowing the player to see and move into another part of the level.
* A home screen to load and save different levels stored locally.
* A settings screen to alter resolution and sound settings (if implemented).

### Won't have

* A fully developed game mechanic with complex AI enemies, multiple levels and missions.
