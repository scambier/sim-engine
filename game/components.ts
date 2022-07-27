import { world } from "./ecs"
import { Color } from "./enums"

export const Ball = world.Component<{ color: Color, radius: number }>()
export const Paddle = world.Component<{ color: Color }>()
export const Collider = world.Component()
export const Position = world.Component<{ x: number, y: number }>()
export const Size = world.Component<{ width: number, height: number }>()
export const Velocity = world.Component<{ dx: number, dy: number }>()
export const MaxVelocity = world.Component<{ dx: number, dy: number }>()