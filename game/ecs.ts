import './libs/extensions'

export type Entity = number
export type ComponentData<T = any> = T extends ComponentFactory
  ? never
  : { [K in keyof T]: T[K] } /*& { _type: ComponentId }*/
export type Inner<X> = X extends ComponentFactory<infer I> ? I : never

type ComponentId = number
type ComponentFactoryContent<T> = T extends ComponentFactory<infer U> ? U : T

/**
 * The Component Factory, used to generate components of the same type
 */
export type ComponentFactory<T = any> = {
  (data?: T): ComponentData<T>
  _type: ComponentId
}

export class World {
  private entityCounter = -1
  private componentFactoryId = -1

  private data: Record<ComponentId, Map<Entity, ComponentData>> = {}
  private queryCache: Map<string, any> = new Map()

  public Component<T>(defaultData?: Partial<T>): ComponentFactory<T> {
    const cmpKey: ComponentId = ++this.componentFactoryId
    this.data[cmpKey] = new Map()

    const fn: ComponentFactory<T> = function (data = {} as any) {
      ; (data as any)._type = cmpKey
      const copy = defaultData ? JSON.parse(JSON.stringify(defaultData)) : {}
      Object.assign(copy, data)
      return copy
    }
    fn._type = cmpKey
    fn.toString = () => {
      return fn._type.toString()
    }
    return fn
  }

  private cleanCache(factories: ComponentId[]) {
    for (const cmpId of factories) {
      for (const key in this.queryCache) {
        const split = key.split('-')
        if (split.indexOf(cmpId.toString()) > -1) {
          this.queryCache.delete(key)
        }
      }
    }
  }

  public spawn(...components: ComponentData[]): Entity {
    const entity = ++this.entityCounter
    this.addComponents(entity, ...components)
    return entity
  }

  public destroy(entity: Entity): void {
    for (const cmpId in this.data) {
      this.cleanCache([Number(cmpId)])
      this.data[cmpId].delete(entity)
    }
  }

  public addComponents(entity: Entity, ...newComponents: ComponentData[]) {
    this.cleanCache(newComponents.map(c => c._type))
    for (let cmp of newComponents) {
      this.data[cmp._type].set(entity, typeof cmp === "function" ? cmp() : cmp)
    }
  }

  public removeComponents(entity: Entity, ...components: ComponentFactory[]) {
    this.cleanCache(components.map(c => c._type))
    for (const cmp of components) {
      this.data[cmp._type].delete(entity)
    }
  }

  /**
   * Returns a single component, or `null` if it doesn't exist
   * @param entity
   * @param factory
   * @returns The component, or null
   */
  public getComponent<T>(
    entity: Entity,
    factory: ComponentFactory<T>
  ): ComponentData<T> | null {

    return (this.data[factory._type].get(entity) as ComponentData<T>) ?? null
  }

  /**
   * Returns several components of an entity.
   *
   * @example world.getComponents(entity, Position, Velocity)
   * @param entity
   * @param factories
   * @returns A sorted array of components
   */
  public getComponents<T extends ReadonlyArray<ComponentFactory>>(
    entity: Entity,
    ...factories: T
  ): { [K in keyof T]: ComponentData<ComponentFactoryContent<T[K]>> | null } {
    return this.getComponentsArr(entity, factories)
  }

  /**
   * Like `getComponents()`, but accepts an array of components instead of a rest parameter.
   * Prefer this method for performances.
   *
   * @example world.getComponentsArr(entity, [Position, Velocity] as const)
   * @param entity
   * @param factories
   * @returns A sorted array of components
   */
  public getComponentsArr<T extends ReadonlyArray<ComponentFactory>>(
    entity: Entity,
    factories: T
  ): { [K in keyof T]: ComponentData<ComponentFactoryContent<T[K]>> | null } {
    return factories.map(f => this.getComponent(entity, f)) as any
  }

  /**
   * Returns the entity id and its entities.<br>
   * The query results are cached, and the cache is updated with added/removed entities/components
   *
   * @example for (const [entity, pos, vel] of world.query(Position, Velocity)) {}
   * @param factories A list of Component factories, as a rest parameter
   * @returns An array of entities with their queried components
   */
  public query<T extends ReadonlyArray<ComponentFactory>>(
    ...factories: T
  ): Array<[Entity, ...{ [K in keyof T]: ComponentFactoryContent<T[K]> }]> {
    return this.queryArr(factories)
  }
  /**
   * Like .query() but avoids `arguments` and so has better performances
   * @example world.queryArr([Position, Rendering] as const)
   * @param factories
   * @returns
   */
  public queryArr<T extends ReadonlyArray<ComponentFactory>>(
    factories: T
  ): Array<[Entity, ...{ [K in keyof T]: ComponentFactoryContent<T[K]> }]> {
    const cacheKey = factories.map((f) => f._type).join("-")
    const cache = this.queryCache.get(cacheKey)
    if (cache) return cache as Array<
      [Entity, ...{ [K in keyof T]: ComponentFactoryContent<T[K]> }]
    >
    const data =
      // 1) Get the entities (ids) that have all queried factories
      this.getEntities(factories)
        // 2) Get the queried components from their factories
        .map(e => ([e, ...this.getComponentsArr(e, factories)]))

    this.queryCache.set(cacheKey, data)
    return data as Array<
      [Entity, ...{ [K in keyof T]: ComponentFactoryContent<T[K]> }]
    >
  }

  public getEntities<T extends ReadonlyArray<ComponentFactory>>(
    factories: T
  ): Entity[] {
    const arrOfKeys = factories.map(f => ([...this.data[f._type].keys()]))
    arrOfKeys.stableSort((a, b) => a.length - b.length)

    let entities = arrOfKeys[0].stableSort()
    for (const item of arrOfKeys) {
      entities = intersection(
        entities.stableSort(),
        item.stableSort()
      )
    }

    return entities.map((e) => Number(e))
  }

  private getComponentsArrUnsafe<T extends ReadonlyArray<ComponentFactory>>(
    entity: Entity,
    factories: T
  ): { [K in keyof T]: ComponentData<ComponentFactoryContent<T[K]>> | null } {
    return factories.map(f => this.data[f._type].get(entity)) as any
  }
}

/**
 * Fast iteration algorithm. Only works on sorted arrays.
 * @param array1
 * @param array2
 * @returns
 */
export function intersection<T>(array1: T[], array2: T[]): T[] {
  const result = []
  // Don't destroy the original arrays
  const a = array1.slice(0)
  const b = array2.slice(0)
  let aLast = a.length - 1
  let bLast = b.length - 1
  while (aLast >= 0 && bLast >= 0) {
    if (a[aLast] > b[bLast]) {
      a.pop()
      aLast--
    } else if (a[aLast] < b[bLast]) {
      b.pop()
      bLast--
    } /* they're equal */ else {
      result.push(a.pop())
      b.pop()
      aLast--
      bLast--
    }
  }
  return result as T[]
}


export const world = new World()