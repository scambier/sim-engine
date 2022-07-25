/**
 * Because the default Array.sort() is not stable, we need to implement our own stable sort.
 * https://medium.com/@fsufitch/is-javascript-array-sort-stable-46b90822543f
 * https://gist.github.com/fsufitch/18bb4692d5f46b649890f8fd58765fbc
 */
interface Comparator<T> {
  (a: T, b: T): number
}
interface Array<T> {
  stableSort(cmp?: Comparator<T>): Array<T>
}

let defaultCmp: Comparator<any> = (a, b) => {
  if (a < b) return -1
  if (a > b) return 1
  return 0
}

Array.prototype.stableSort = function <T>(
  cmp: Comparator<T> = defaultCmp
): T[] {
  let self: T[] = this // for typing
  let stabilized = self.map((el, index) => <[T, number]>[el, index])
  let stableCmp: Comparator<[T, number]> = (a, b) => {
    let order = cmp(a[0], b[0])
    if (order != 0) return order
    return a[1] - b[1]
  }

  stabilized.sort(stableCmp)
  for (let i = 0; i < self.length; i++) {
    self[i] = stabilized[i][0]
  }

  return self
}
