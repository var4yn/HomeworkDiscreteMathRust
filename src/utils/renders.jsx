
export function OkRender () {
    return (
        <div className="text-green-600">
            ok
        </div>
    )
}

export function InvalidRender ()  {
    return (
        <div className="text-red-600">
            invalid
        </div>
    )
}

export function MxArgRender  ({value})  {
    const message = `valid max value argument: ${value}`;
    return (
        <div>
            {message}
        </div>
    )
}

export function ErrorRender( {err} ) {

    return (
        <div className="text-red-500">
            {`Ошибка: ${err}`}
        </div>
    )
}

export function OutputRender({output}) {
    return (
        <div>
            {output}
        </div>
    )
}