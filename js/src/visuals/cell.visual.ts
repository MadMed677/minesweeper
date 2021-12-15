import * as PIXI from 'pixi.js'
import {IVisual} from './visual.interface'
import {CellStatus, WasmCType, WasmCTypeName} from '@minesweeper/engine'

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
	ctype: WasmCType
}

export class CellVisual implements IVisual<ICellVisualProps> {
	private readonly id: number;
	private props!: ICellVisualProps;

	readonly graphics = new PIXI.Graphics()
	private readonly text = new PIXI.Text('', {
		fontSize: 16,
		wordWrap: true,
		wordWrapWidth: 50,
		align: 'center',
	})
	private readonly sprite: PIXI.Sprite

	constructor(id: number) {
		this.id = id;

		/** Create an Sprite with empty texture */
		this.sprite = new PIXI.Sprite()

		this.graphics.addChild(this.text)
		this.graphics.addChild(this.sprite)
	}

	public setProps(props: Partial<ICellVisualProps>): void {
		// If it's a first render
		if (!this.props) {
			// @ts-ignore In a first render we have full props
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
			nextProps.status !== this.props.status ||
			nextProps.ctype.name !== this.props.ctype.name ||
			nextProps.ctype.value !== this.props.ctype.value
		);
	}

	public render(): void {
		this.graphics.clear();

		if (this.props.status === CellStatus.Uncovered) {
			this.graphics.beginFill(PIXI.utils.string2hex('#D5F5E3'))

			this.graphics.interactive = false
			this.graphics.buttonMode = false

			if (this.props.ctype.name === WasmCTypeName.Mine) {
				this.sprite.texture = PIXI.Texture.from('bomb')
			} else {
				this.sprite.texture = PIXI.Texture.from('empty_selected')
			}

		} else if (this.props.status === CellStatus.Hidden) {
			this.graphics.beginFill(PIXI.utils.string2hex('#FCF3CF'))

			this.graphics.interactive = true
			this.graphics.buttonMode = true

			this.sprite.texture = PIXI.Texture.from('empty_not_selected')
		}

		this.graphics.x = this.props.position.x
		this.graphics.y = this.props.position.y

		this.sprite.width = this.props.size.width
		this.sprite.height = this.props.size.height

		this.graphics.drawRect(
			0,
			0,
			this.props.size.width,
			this.props.size.height,
		)

		this.graphics.endFill()

		this.graphics.name = String(this.id)
	}
}
