import { Element } from "../element.ts";
import { State } from "../state.ts";

export class TextElement extends Element {
    public _text = ""

    public set text(val: string) {
        this._text = val
        this.set(State.Text, val) 
    }

    public get text(): string {
        return this._text
    }
}