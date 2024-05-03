import { InvalidRender, OkRender } from "../utils/renders";

const OkOrInvalidComponent = ({condition}) => {

    return (
        <>
            {condition === false ? <InvalidRender/> : <OkRender/>}
        </>
    )

}

export default OkOrInvalidComponent;