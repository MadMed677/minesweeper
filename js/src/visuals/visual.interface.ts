import PIXI from 'pixi.js'

export interface IVisualConstructor<T extends IVisual<T>> {
	new (id: number): T
}

export interface IVisual<T> {
	readonly graphics: PIXI.DisplayObject
	shouldComponentUpdate(nextProps: T): boolean
	setProps(props: Partial<T>): void;
	render(): void
	destroy?(): void
}
