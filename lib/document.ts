import { library } from "./bindings.ts";
import { Element, ElementType } from "./element.ts";
import { DivElement } from "./elements/div.ts";
import { ElementId } from "./util/util.ts";

export class Document extends Element {
    public _rid: number
    public body: DivElement

    constructor(title: string) {
        super(ElementType.Div)
        this._rid = ElementId()
        this.body = new DivElement()

        this.open(title)
    }

    public async open(title: string) {
        const encoder = new TextEncoder()
        const buffer = encoder.encode(title);
        await library.symbols.ops_create_window(this._rid, buffer, buffer.length)
    }
}