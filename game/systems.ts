import { Paddle, Position, Size, Ball, Velocity, MaxVelocity } from "./components"
import * as ECS from "./ecs"
import { KeyCode } from "./enums"
import { round, collisionRectCircle, clamp } from "./utils"

export function sysDraw(world: ECS.World) {
  {
    // Paddle
    const [_e, paddle, pos, size] = world.query(Paddle, Position, Size)[0]
    drawRectFill(pos.x, pos.y, size.width, size.height, paddle.color)
  }

  {
    // Ball
    const [_e, ball, pos] = world.query(Ball, Position)[0]
    drawCircFill(pos.x, pos.y, ball.radius, ball.color)
  }
}

export function sysApplyVelocity(world: ECS.World) {
  for (const [e, pos, vel] of world.query(Position, Velocity)) {
    pos.x += vel.dx
    pos.y += vel.dy

    const maxVel = world.getComponent(e, MaxVelocity)
    if (maxVel) {
      if (Math.abs(vel.dx) > maxVel.dx) {
        vel.dx *= 0.99
        vel.dx = round(vel.dx, 2)
        // trace(vel.dx)
      }
      if (Math.abs(vel.dy) > maxVel.dy) {
        vel.dy *= 0.99
      }
    }
    const [paddle, size] = world.getComponents(e, Paddle, Size)
    if (paddle && size) {
      if (pos.x < 0) {
        pos.x = 0
      }
      if (pos.x + size.width > WIDTH) {
        pos.x = WIDTH - size.width
      }
    }
  }
}

export function sysBounceCollide(world: ECS.World) {
  // Ball
  const [_b, ball, ballPos, ballVel] = world.query(Ball, Position, Velocity)[0]
  // Paddle
  const [_p, paddle, paddlePos, paddleSize, paddleVel] = world.query(Paddle, Position, Size, Velocity)[0]

  if (!paddle || !ball) {
    trace("No paddle or ball")
    return
  }

  /*
   * Wall collision
   */

  // Left wall
  if (ballPos.x - ball.radius < 0) {
    ballPos.x = ball.radius
    ballVel.dx = Math.abs(ballVel.dx)
  }
  // Right wall
  if (ballPos.x + ball.radius > WIDTH) {
    ballPos.x = WIDTH - ball.radius
    ballVel.dx = -Math.abs(ballVel.dx)
  }
  // Top wall
  if (ballPos.y - ball.radius < 0) {
    ballPos.y = ball.radius
    ballVel.dy = Math.abs(ballVel.dy)
  }
  // Bottom wall
  if (ballPos.y + ball.radius > HEIGHT) {
    ballPos.y = HEIGHT - ball.radius
    ballVel.dy = -Math.abs(ballVel.dy)
  }

  /*
   * Paddle collision
   */

  // Get next ball position
  const nextBallPos = { x: ballPos.x + ballVel.dx, y: ballPos.y + ballVel.dy }
  // Get next paddle position
  const nextPaddlePos = { x: paddlePos.x + paddleVel.dx, y: paddlePos.y + paddleVel.dy }

  // If the ball and paddle will collide next frame
  if (collisionRectCircle(
    { x: nextPaddlePos.x, y: nextPaddlePos.y, w: paddleSize.width, h: paddleSize.height },
    { x: nextBallPos.x, y: nextBallPos.y, r: ball.radius }
  )) {
    // Top side
    if (ballPos.y < paddlePos.y) {
      ballVel.dy = -Math.abs(ballVel.dy)
    }
    // Left side
    else if (ballPos.x < paddlePos.x) {
      ballVel.dx = -Math.abs(ballVel.dx)
    }
    // Right side
    else if (ballPos.x > paddlePos.x + paddleSize.width) {
      ballVel.dx = Math.abs(ballVel.dx)
    }
    // Bottom side
    else {
      ballVel.dy = Math.abs(ballVel.dy)
    }

    // Apply the paddle velocity on the ball
    ballVel.dx += paddleVel.dx
    ballVel.dy += paddleVel.dy

    playAudio('bump_wall.wav')
  }
}

export function sysInput(world: ECS.World) {
  const [_e, vel] = world.query(Velocity, Paddle)[0]
  let pressed = false
  if (isKeyDown(KeyCode.LEFT)) {
    pressed = true
    vel.dx = -3
  }
  if (isKeyDown(KeyCode.RIGHT)) {
    pressed = true
    vel.dx = 3
  }
  // if (isKeyDown(KeyCode.UP)) {
  //     pressed = true
  //     vel.dy = -3
  // }
  // if (isKeyDown(KeyCode.DOWN)) {
  //     pressed = true
  //     vel.dy = 3
  // }
  if (!pressed) {
    vel.dx /= 1.7
    vel.dy /= 1.7
  }
  vel.dx = clamp(vel.dx, -5, 5)
  if (Math.abs(vel.dx) < 0.01) {
    vel.dx = 0
  }
}