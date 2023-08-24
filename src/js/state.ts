import { atom } from "recoil";
import { Airport } from "./airport";

export const airportsState = atom({
    key: 'airportsState',
    default: [] as Airport[]
});