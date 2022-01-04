import * as PIXI from 'pixi.js';
import {IVisual} from './visual.interface';
import {WasmCellState, WasmCType, WasmCTypeName} from '@minesweeper/engine';

export interface ICellVisualProps {
    position: {
        x: number;
        y: number;
    };
    size: {
        width: number;
        height: number;
    };
    status: WasmCellState;
    ctype: WasmCType;
}

export class CellVisual implements IVisual<ICellVisualProps> {
    private readonly id: number;
    private props!: ICellVisualProps;

    readonly graphics = new PIXI.Container();
    private readonly sprite: PIXI.Sprite;

    constructor(id: number) {
        this.id = id;

        /** Create an Sprite with empty texture */
        this.sprite = new PIXI.Sprite();

        this.graphics.addChild(this.sprite);
    }

    public setProps(props: Partial<ICellVisualProps>): void {
        // If it's a first render
        if (!this.props) {
            this.props = props as ICellVisualProps;

            return;
        }

        this.props = Object.assign(this.props, props);
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
        if (this.props.status === WasmCellState.Revealed) {
            this.graphics.interactive = false;
            this.graphics.buttonMode = false;

            if (this.props.ctype.name === WasmCTypeName.Mine) {
                this.sprite.texture = PIXI.Texture.from('bomb');
            } else {
                if (this.props.ctype.value > 0) {
                    this.sprite.texture = PIXI.Texture.from(
                        `mark_${this.props.ctype.value}`,
                    );
                } else {
                    this.sprite.texture = PIXI.Texture.from('empty_selected');
                }
            }
        } else if (this.props.status === WasmCellState.Hidden) {
            this.graphics.interactive = true;
            this.graphics.buttonMode = true;

            this.sprite.texture = PIXI.Texture.from('empty_not_selected');
        } else if (this.props.status === WasmCellState.Flagged) {
            this.graphics.interactive = true;
            this.graphics.buttonMode = true;

            this.sprite.texture = PIXI.Texture.from('flagged');
        }

        this.graphics.x = this.props.position.x;
        this.graphics.y = this.props.position.y;

        this.sprite.width = this.props.size.width;
        this.sprite.height = this.props.size.height;

        this.graphics.name = String(this.id);
    }
}
