export type ElementEvent<T extends HTMLElement> = MouseEvent & {
  currentTarget: EventTarget & T;
};