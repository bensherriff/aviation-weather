import { airports } from "@/js/state";
import Link from "next/link";


export default function Page({ params }: { params: { icao: string } }) {
    const airport = airports.get(params.icao);
    return <>
        <div className="border-b border-gray-200 bg-gray-400 px-4 py-5 sm:px-6 flex justify-between">
            <h3 className="text-base font-semibold leading-6 text-gray-900">{airport?.name}</h3>
            <Link href={"/"}>Back</Link>
        </div>
    </>;
}