// Naive implementation
export function ElementId(): number {
    return Math.floor(Math.random()*1e10) >>> 0;
}