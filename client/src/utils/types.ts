export type ElementEvent<EV extends UIEvent, EL extends HTMLElement> = EV & {
  currentTarget: EventTarget & EL;
};