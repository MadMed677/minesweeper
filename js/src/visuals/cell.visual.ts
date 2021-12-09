import * as PIXI from 'pixi.js'
import {IVisual} from './visual.interface'
import {CellStatus} from '@minesweeper/engine'

export interface ICellVisualProps {
	position: {
		x: number
		y: number
	}
	size: {
		width: number
		height: number
	}
	status: CellStatus
}

export class CellVisual implements IVisual<ICellVisualProps> {
	private readonly id: number;
	private props!: ICellVisualProps;

	readonly graphics = new PIXI.Graphics()

	constructor(id: number) {
		this.id = id;
	}

	public setProps(props: Partial<ICellVisualProps>): void {
		// If it's a first render
		if (!this.props) {
			// @ts-ignore
			this.props = props

			return
		}

		this.props = Object.assign(this.props, props)
	}

	public shouldComponentUpdate(nextProps: ICellVisualProps): boolean {
		// If it's a first render
		if (!this.props) {
			return true;
		}

		return (
			nextProps.position.x !== this.props.position.x ||
			nextProps.position.y !== this.props.position.y ||
			nextProps.size.width !== this.props.size.width ||
			nextProps.size.height !== this.props.size.height ||
			nextProps.status !== this.props.status
		);
	}

	public render(): void {
		this.graphics.clear();

		if (this.props.status === CellStatus.Uncovered) {
			this.graphics.beginFill(PIXI.utils.string2hex('#D5F5E3'))

			this.graphics.interactive = false
			this.graphics.buttonMode = false
		} else if (this.props.status === CellStatus.Hidden) {
			this.graphics.beginFill(PIXI.utils.string2hex('#FCF3CF'))

			this.graphics.interactive = true
			this.graphics.buttonMode = true
		}

		this.graphics.drawRect(
			this.props.position.x,
			this.props.position.y,
			this.props.size.width,
			this.props.size.height,
		)

		this.graphics.endFill()

		this.graphics.name = String(this.id)
	}
}
