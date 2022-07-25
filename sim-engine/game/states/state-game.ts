import { World } from "../ecs"
import { GameState } from "../libs/state-manager"
import { sysInput, sysBounceCollide, sysApplyVelocity, sysDraw } from "../systems"

export function StateGame(world: World): GameState {
    return {
        enter() {

        },
        update() {
            sysInput(world)
            sysBounceCollide(world)
            sysApplyVelocity(world)
        },
        draw() {
            clearScreen()

            sysDraw(world)
        },
    }
}