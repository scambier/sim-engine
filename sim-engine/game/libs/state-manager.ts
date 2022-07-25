import { world } from "../ecs"
import { StateGame } from "../states/state-game"
import { StateMenu } from "../states/state-menu"

export interface GameState {
  enter: (data?: any) => void
  update: () => void
  draw: () => void
  leave?: () => void
}

class StateManager {
  private stack: GameState[] = []

  private getCurrentState(): null | GameState {
    return this.stack[this.stack.length - 1] ?? null
  }

  private setCurrentState(state: GameState) {
    if (this.stack.length === 0) {
      this.stack = [state]
    } else {
      const last = this.stack[this.stack.length - 1]
      this.stack[this.stack.length - 1]
      if (last.leave) last.leave
      this.stack[this.stack.length - 1] = state
    }
  }

  /**
   * Replaces the current state
   * @param state
   */
  public changeState(state: GameState, data?: any) {
    const current = this.getCurrentState()
    if (current?.leave) current.leave()
    this.setCurrentState(state)
    state.enter(data)
  }

  /**
   * Pushes a new state on the stack
   * @param state
   */
  public pushState(state: GameState, data?: any) {
    this.stack.push(state)
    state.enter(data)
  }

  /**
   * Pops the top state
   */
  public popState() {
    const state = this.stack.pop()
    if (state?.leave) state.leave()
  }

  /**
   * Updates the top state
   */
  public update() {
    const current = this.getCurrentState()
    if (current) current.update()
  }

  /**
   * Draws all states
   */
  public draw() {
    for (const state of this.stack) {
      state.draw()
    }
  }
}

export const stateManager = new StateManager()

export const states = {
  menu: StateMenu(world),
  game: StateGame(world),
} as const
stateManager.changeState(states.menu)