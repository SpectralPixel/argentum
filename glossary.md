# Glossary

## Abbreviations

- Coord = Coordinate

## World data

### Coordinate systems

In the representation of world data, there are three types of coordinate
systems:

- Global coordinates
- Local coordinates
- Chunk coordinates

In the realm of position data, there is also some nuance as to what data is
expected. There are currently two types of position data: positions and axes.

#### Global Coordinates

Global coordinates represent an absolute position in the world's coordinate
system. In Bevy, object's positions will generally be global coordinates.

#### Local Coordinates

Local coordinates represent any Voxel's position within a Chunk. It's only ever
used internally.

#### Chunk Coordinates

Chunk coordinates are the coordinates of the Chunks as stored by the World.

#### Positions vs. Axes

A function that operates on position data may call a function which expects an
"axis" as an input. Every component of a coordinate lies on a axis, and
therefore this word is used to describe when a function operates on only a
single component of a coordinate.

#### Terminology

To describe location data, there are two terms that will be commonly used in
the code.

First, "position". The word position is used exclusively in variable names for
consistency. A position can either represent a global, local or chunk
coordinate. To annotate which coordinate system is used (as currently there
are no explicit types for these), please use the appropriate system as a
prefix (e.g. global_position if the position represents global coordinates).

Second, "coordinate". For the sake of consistency, this word is only used in
types that represent these different coordinate systems or functions that
convert between them.
