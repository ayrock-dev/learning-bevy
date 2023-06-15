# learning-bevy

## Resources

This project is built on the open source Bevy game engine. Bevy has some [Getting Started documentation here.](https://bevyengine.org/learn/book/introduction/)

## Tools

### Physics

Rather than building physics systems from scratch, we use the Rapier physics library. The [Rapier Bevy Plugin has pretty good documentation here.](https://rapier.rs/docs/user_guides/bevy_plugin/getting_started_bevy/)

### Level Design

Bevy does not have a level editor yet (a 3D world view like Blender, Unity, SolidWorks etc). This means that if we want to visually design levels, we have to use a third party program.

[Level Design Toolkit (Ldtk)](https://ldtk.io/) is a 2d level editor designed by the creator of Dead Cells. There is also a Ldtk Bevy Plugin ([bevy_ecs_ldtk](https://github.com/Trouv/bevy_ecs_ldtk)) which has the capability to automatically load a `.ldtk` project file and render it in-game.
