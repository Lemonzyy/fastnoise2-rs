/* automatically generated by rust-bindgen 0.69.4 */

extern "C" {
    pub fn fnNewFromEncodedNodeTree(
        encodedString: *const ::std::os::raw::c_char,
        simdLevel: ::std::os::raw::c_uint,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn fnDeleteNodeRef(node: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn fnGetSIMDLevel(node: *const ::std::os::raw::c_void) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn fnGetMetadataID(node: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fnGenUniformGrid2D(
        node: *const ::std::os::raw::c_void,
        noiseOut: *mut f32,
        xStart: ::std::os::raw::c_int,
        yStart: ::std::os::raw::c_int,
        xSize: ::std::os::raw::c_int,
        ySize: ::std::os::raw::c_int,
        frequency: f32,
        seed: ::std::os::raw::c_int,
        outputMinMax: *mut f32,
    );
}
extern "C" {
    pub fn fnGenUniformGrid3D(
        node: *const ::std::os::raw::c_void,
        noiseOut: *mut f32,
        xStart: ::std::os::raw::c_int,
        yStart: ::std::os::raw::c_int,
        zStart: ::std::os::raw::c_int,
        xSize: ::std::os::raw::c_int,
        ySize: ::std::os::raw::c_int,
        zSize: ::std::os::raw::c_int,
        frequency: f32,
        seed: ::std::os::raw::c_int,
        outputMinMax: *mut f32,
    );
}
extern "C" {
    pub fn fnGenUniformGrid4D(
        node: *const ::std::os::raw::c_void,
        noiseOut: *mut f32,
        xStart: ::std::os::raw::c_int,
        yStart: ::std::os::raw::c_int,
        zStart: ::std::os::raw::c_int,
        wStart: ::std::os::raw::c_int,
        xSize: ::std::os::raw::c_int,
        ySize: ::std::os::raw::c_int,
        zSize: ::std::os::raw::c_int,
        wSize: ::std::os::raw::c_int,
        frequency: f32,
        seed: ::std::os::raw::c_int,
        outputMinMax: *mut f32,
    );
}
extern "C" {
    pub fn fnGenPositionArray2D(
        node: *const ::std::os::raw::c_void,
        noiseOut: *mut f32,
        count: ::std::os::raw::c_int,
        xPosArray: *const f32,
        yPosArray: *const f32,
        xOffset: f32,
        yOffset: f32,
        seed: ::std::os::raw::c_int,
        outputMinMax: *mut f32,
    );
}
extern "C" {
    pub fn fnGenPositionArray3D(
        node: *const ::std::os::raw::c_void,
        noiseOut: *mut f32,
        count: ::std::os::raw::c_int,
        xPosArray: *const f32,
        yPosArray: *const f32,
        zPosArray: *const f32,
        xOffset: f32,
        yOffset: f32,
        zOffset: f32,
        seed: ::std::os::raw::c_int,
        outputMinMax: *mut f32,
    );
}
extern "C" {
    pub fn fnGenPositionArray4D(
        node: *const ::std::os::raw::c_void,
        noiseOut: *mut f32,
        count: ::std::os::raw::c_int,
        xPosArray: *const f32,
        yPosArray: *const f32,
        zPosArray: *const f32,
        wPosArray: *const f32,
        xOffset: f32,
        yOffset: f32,
        zOffset: f32,
        wOffset: f32,
        seed: ::std::os::raw::c_int,
        outputMinMax: *mut f32,
    );
}
extern "C" {
    pub fn fnGenTileable2D(
        node: *const ::std::os::raw::c_void,
        noiseOut: *mut f32,
        xSize: ::std::os::raw::c_int,
        ySize: ::std::os::raw::c_int,
        frequency: f32,
        seed: ::std::os::raw::c_int,
        outputMinMax: *mut f32,
    );
}
extern "C" {
    pub fn fnGenSingle2D(
        node: *const ::std::os::raw::c_void,
        x: f32,
        y: f32,
        seed: ::std::os::raw::c_int,
    ) -> f32;
}
extern "C" {
    pub fn fnGenSingle3D(
        node: *const ::std::os::raw::c_void,
        x: f32,
        y: f32,
        z: f32,
        seed: ::std::os::raw::c_int,
    ) -> f32;
}
extern "C" {
    pub fn fnGenSingle4D(
        node: *const ::std::os::raw::c_void,
        x: f32,
        y: f32,
        z: f32,
        w: f32,
        seed: ::std::os::raw::c_int,
    ) -> f32;
}
extern "C" {
    pub fn fnGetMetadataCount() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fnGetMetadataName(id: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn fnNewFromMetadata(
        id: ::std::os::raw::c_int,
        simdLevel: ::std::os::raw::c_uint,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn fnGetMetadataVariableCount(id: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fnGetMetadataVariableName(
        id: ::std::os::raw::c_int,
        variableIndex: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn fnGetMetadataVariableType(
        id: ::std::os::raw::c_int,
        variableIndex: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fnGetMetadataVariableDimensionIdx(
        id: ::std::os::raw::c_int,
        variableIndex: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fnGetMetadataEnumCount(
        id: ::std::os::raw::c_int,
        variableIndex: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fnGetMetadataEnumName(
        id: ::std::os::raw::c_int,
        variableIndex: ::std::os::raw::c_int,
        enumIndex: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn fnSetVariableFloat(
        node: *mut ::std::os::raw::c_void,
        variableIndex: ::std::os::raw::c_int,
        value: f32,
    ) -> bool;
}
extern "C" {
    pub fn fnSetVariableIntEnum(
        node: *mut ::std::os::raw::c_void,
        variableIndex: ::std::os::raw::c_int,
        value: ::std::os::raw::c_int,
    ) -> bool;
}
extern "C" {
    pub fn fnGetMetadataNodeLookupCount(id: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fnGetMetadataNodeLookupName(
        id: ::std::os::raw::c_int,
        nodeLookupIndex: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn fnGetMetadataNodeLookupDimensionIdx(
        id: ::std::os::raw::c_int,
        nodeLookupIndex: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fnSetNodeLookup(
        node: *mut ::std::os::raw::c_void,
        nodeLookupIndex: ::std::os::raw::c_int,
        nodeLookup: *const ::std::os::raw::c_void,
    ) -> bool;
}
extern "C" {
    pub fn fnGetMetadataHybridCount(id: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fnGetMetadataHybridName(
        id: ::std::os::raw::c_int,
        hybridIndex: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn fnGetMetadataHybridDimensionIdx(
        id: ::std::os::raw::c_int,
        hybridIndex: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fnSetHybridNodeLookup(
        node: *mut ::std::os::raw::c_void,
        hybridIndex: ::std::os::raw::c_int,
        nodeLookup: *const ::std::os::raw::c_void,
    ) -> bool;
}
extern "C" {
    pub fn fnSetHybridFloat(
        node: *mut ::std::os::raw::c_void,
        hybridIndex: ::std::os::raw::c_int,
        value: f32,
    ) -> bool;
}
