import { World } from "../ecs"
import { Color, KeyCode } from "../enums"
import { GameState, stateManager, states } from "../libs/state-manager"

export function StateMenu(world: World): GameState {
    return {
        enter() {

        },
        update() {
            if (isKeyJustPressed(KeyCode.X)) {
                stateManager.changeState(states.game)
            }
        },
        draw() {
            clearScreen(15)
            print("PICO HERO BREAKOUT", 30, 40, Color.LightGreen)
            print("PRESS [x] TO START", 32, 80, Color.LightBlue)
        },
    }
}