import React, { useState } from 'react'
import { Input, Field, Label, Description, Button } from '@headlessui/react'
import axios from 'axios'
import { stringify } from 'postcss'


export function DeshortenerForm() {
    const [tinifiedUrl, setTinifiedUrl] = useState<string>("")
    const [detinifiedUrl, setDetinifiedUrl] = useState<string>("")

    const detinyUrl = async (e: React.FormEvent<HTMLFormElement>) => {
        const fdata = new URLSearchParams();
        fdata.append("tinified_url", tinifiedUrl);
        let result = await fetch(
            "http://0.0.0.0:8000/tinify",
            {
                method: "POST",
                body: fdata.toString(),
                headers: {"Content-Type": "application/x-ww-form-urlencoded"}
            }).then(async (d) => await d.json());
    }

    return (
        <form onSubmit={(e)=>detinyUrl(e)}>
            <div className="flex flex-col items-center justify-center border border-white p-4">
                <div className="flex flex-col items-center justify-center">
                    <Input type="text" name="url" value={tinifiedUrl} onChange={(e)=>setTinifiedUrl(e.target?.value)}/>
                    <Button type="submit" className="rounded bg-sky-600 px-4 py-2 text-sm text-white data-active:bg-sky-700 data-hover:bg-sky-500"> Detinify URL </Button>
                </div>
                
                <div className="px-2">
                    <p className="px-2"> Detinified URL: </p>
                    <Input type="text" readOnly name="url" value={detinifiedUrl}/>
                </div>
            </div>
        </form>
    )

}


export function ShortenerForm() {
    const [url, setUrl] = useState<string>('')
    const [tinifiedUrl, setTinifiedUrl] = useState<string>('')
    

    const tinyUrl = async (e: React.FormEvent<HTMLFormElement>) => {
        console.log(`running tinyUrl routine: ${e.currentTarget}`)
        e.preventDefault();
        console.log(`event: ${e.currentTarget}`)
        console.log(`VITE_API_URL: ${import.meta.env.VITE_API_HOST}`)
        const fdata = new URLSearchParams()
        // note: can also do const fdata = new FormData(e.currentTarget)
        fdata.append("url", url)
        
        
        let result = await fetch(
                'http://0.0.0.0:8000/tinify', {
                method: "POST",
                body: fdata.toString(), // converting form data to toString should work with application/x-ww-form-urlencoded
                headers: {"Content-Type": "application/x-www-form-urlencoded"} // note for rust backend it is more convenient to use this MIME type rather than multipart/form-data
            }).then( async (d) => await d.json() );

        setTinifiedUrl(result.tinified_url)
    }

    return (
        <form onSubmit={(e) => tinyUrl(e) }>
            <div className="flex flex-col items-center justify-center border border-white p-4">
                <div className="flex flex-col items-center justify-center pb-3">
                    <Input type="text" name="url" value={url} onChange={(e) =>{ setUrl(e.target.value) }}/>
                    <Button type="submit" className="rounded bg-sky-600 px-4 py-2 text-sm text-white data-active:bg-sky-700 data-hover:bg-sky-500">
                        Tinify URL
                    </Button>
                </div>
                
                <div className="px-2">
                    <p className="px-2"> Tinified URL: </p>
                    <Input type="text" readOnly name="url" value={tinifiedUrl}/>
                </div>
            </div>
        </form>
    )


}