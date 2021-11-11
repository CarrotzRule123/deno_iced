import { Element, ElementType } from "../element.ts";
import { State } from "../state.ts";

export class ButtonElement extends Element {
    public _text = ""

    constructor() {
        super(ElementType.Button)
    }

    public set text(val: string) {
        this._text = val
        this.set(State.Text, val) 
    }

    public get text(): string {
        return this._text
    }
}