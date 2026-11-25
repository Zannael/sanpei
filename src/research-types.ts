export type ResearchParameter = {
 id:string; label:string; kind:'number'|'boolean'|'category'; source:string;
 unit?:string|null; options?:string[]; minimum?:number|null; maximum?:number|null;
};
