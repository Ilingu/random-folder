export interface Response<T> {
  succeed: boolean;
  data?: T;
  reason?: string;
}
export const decodeReponse = <T = unknown>(resp: string): Response<T> => {
  try {
    return JSON.parse(resp);
  } catch (err) {
    return { succeed: false };
  }
};
