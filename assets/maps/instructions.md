## Map file structure

```
camera.x camera.y camera.z look_at.x look_at.y look_at.z
player.x player.y player.z yaw pitch
number_of_walls
id start.x start.y start.z end.x end.y end.z height scalar.u scalar.v offset.u offset.v rotation.u rotation.v texture_id
number_of_floors
id a.x a.y a.z b.x b.y b.z c.x c.y c.z scalar.u scalar.v offset.u offset.v rotation.u rotation.v world_aligned_uv (bool as integer) texture_id
number_of_enemies
id position.x position.y position.z
```

## Example usage

```
0 0 0 0 0 -1
0 5 0 0 0
2
0 -25 0 -50 25 0 -50 10 1 1 0 0 0 1
1 25 0 -50 25 0 -75 10 1 1 0 0 0 1
1
2 -25 0 -50 25 0 -50 25 0 -75 1 1 0 0 0 1 0
1
0 5 5 -5
```