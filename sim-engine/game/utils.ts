export function clamp(value: number, min: number, max: number): number {
    return Math.min(Math.max(value, min), max);
}

export function collisionRectCircle(rect: { x: number, y: number, w: number, h: number }, circ: { x: number, y: number, r: number }): boolean {
    // Check if rect and circle collide
    if (circ.x + circ.r < rect.x ||
        circ.x - circ.r > rect.x + rect.w ||
        circ.y + circ.r < rect.y ||
        circ.y - circ.r > rect.y + rect.h) {
        return false;
    }
    return true
}

export function round(value: number, decimal: number): number {
    return Math.round(value * Math.pow(10, decimal)) / Math.pow(10, decimal);
}