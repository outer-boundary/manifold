export type TabType = "domains" | "friends" | "settings" | "logout";

export interface TabInfo { 
  name: TabType;
  icon: string 
}