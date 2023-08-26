export const airportsState = atom({
    key: 'airportsState',
    default: [] as Airport[]
});

import { Airport } from "@/js/airport";
import { atom } from "recoil";