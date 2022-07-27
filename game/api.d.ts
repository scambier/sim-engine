declare const WIDTH: number
declare const HEIGHT: number
declare function trace(str: any): void
declare function getDelta(): number

declare function clearScreen(color?: number): void
declare function print(txt: any, x: number, y: number, color: number, border?: number): void
declare function drawRect(x: number, y: number, w: number, h: number, color: number): void
declare function drawRectFill(x: number, y: number, w: number, h: number, color: number): void
declare function drawCirc(x: number, y: number, r: number, color: number): void
declare function drawCircFill(x: number, y: number, r: number, color: number): void

declare function isKeyDown(key: number): boolean
declare function isKeyJustPressed(key: number): boolean

declare function playAudio(audio: string): void