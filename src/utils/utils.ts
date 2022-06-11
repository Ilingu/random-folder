import { FunctionJobs } from "./interfaces/interfaces";

// Local Storage
export const SetLSItem = (key: string, val: any) => {
  window.localStorage.setItem(key, JSON.stringify(val));
};

export const GetLSItem = (key: string): FunctionJobs<any> => {
  if (!window?.localStorage || !key) return { success: false };

  const resp = window.localStorage.getItem(key);
  if (!resp) return { success: false };

  let value: any = resp;
  try {
    value = JSON.parse(resp);
  } catch (err) {}

  return { success: true, returns: value };
};

// String Check
export const EmptyString = (str: string) => {
  if (typeof str !== "string" || str.trim().length <= 0) return true;
  return false;
};
