import { ErrorRender, OutputRender } from "../utils/renders"

const OutputComponent = ({err, output}) => {

    return (
        <>
            { err.length > 0 ? <ErrorRender err={err}/> : <OutputRender output={output}/> }
        </>
    )

}

export default OutputComponent
