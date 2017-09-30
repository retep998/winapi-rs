// Copyright © 2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::{c_double, c_long, c_uint};
use shared::minwindef::{BOOL, DWORD};
use shared::wtypes::{BSTR, VARIANT_BOOL};
use um::oaidl::{IDispatch, IDispatchVtbl, VARIANT};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::HRESULT;
STRUCT!{struct XML_ERROR {
    _nLine: c_uint,
    _pchBuf: BSTR,
    _cchBuf: c_uint,
    _ich: c_uint,
    _pszFound: BSTR,
    _pszExpected: BSTR,
    _reserved1: DWORD,
    _reserved2: DWORD,
}}
ENUM!{enum DOMNodeType {
    NODE_INVALID = 0,
    NODE_ELEMENT = NODE_INVALID + 1,
    NODE_ATTRIBUTE = NODE_ELEMENT + 1,
    NODE_TEXT = NODE_ATTRIBUTE + 1,
    NODE_CDATA_SECTION = NODE_TEXT + 1,
    NODE_ENTITY_REFERENCE = NODE_CDATA_SECTION + 1,
    NODE_ENTITY = NODE_ENTITY_REFERENCE + 1,
    NODE_PROCESSING_INSTRUCTION = NODE_ENTITY + 1,
    NODE_COMMENT = NODE_PROCESSING_INSTRUCTION + 1,
    NODE_DOCUMENT = NODE_COMMENT + 1,
    NODE_DOCUMENT_TYPE = NODE_DOCUMENT + 1,
    NODE_DOCUMENT_FRAGMENT = NODE_DOCUMENT_TYPE + 1,
    NODE_NOTATION = NODE_DOCUMENT_FRAGMENT + 1,
}}
ENUM!{enum XMLELEM_TYPE {
    XMLELEMTYPE_ELEMENT = 0,
    XMLELEMTYPE_TEXT = XMLELEMTYPE_ELEMENT + 1,
    XMLELEMTYPE_COMMENT = XMLELEMTYPE_TEXT + 1,
    XMLELEMTYPE_DOCUMENT = XMLELEMTYPE_COMMENT + 1,
    XMLELEMTYPE_DTD = XMLELEMTYPE_DOCUMENT + 1,
    XMLELEMTYPE_PI = XMLELEMTYPE_DTD + 1,
    XMLELEMTYPE_OTHER = XMLELEMTYPE_PI + 1,
}}
RIDL!(#[uuid(0x2933bf8f, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60)]
interface IXMLDOMImplementation(IXMLDOMImplementationVtbl): IDispatch(IDispatchVtbl) {
    fn hasFeature(
        feature: BSTR,
        version: BSTR,
        hasFeature: *mut VARIANT_BOOL,
    ) -> HRESULT,
});
RIDL!(#[uuid(0x2933bf80, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60)]
interface IXMLDOMNode(IXMLDOMNodeVtbl): IDispatch(IDispatchVtbl) {
    fn get_nodeName(
        name: *mut BSTR,
    ) -> HRESULT,
    fn get_nodeValue(
        value: *mut VARIANT,
    ) -> HRESULT,
    fn put_nodeValue(
        value: VARIANT,
    ) -> HRESULT,
    fn get_nodeType(
        type_: *mut DOMNodeType,
    ) -> HRESULT,
    fn get_parentNode(
        parent: *mut *mut IXMLDOMNode,
    ) -> HRESULT,
    fn get_childNodes(
        childList: *mut *mut IXMLDOMNodeList,
    ) -> HRESULT,
    fn get_firstChild(
        firstChild: *mut *mut IXMLDOMNode,
    ) -> HRESULT,
    fn get_lastChild(
        lastChild: *mut *mut IXMLDOMNode,
    ) -> HRESULT,
    fn get_previousSibling(
        previousSibling: *mut *mut IXMLDOMNode,
    ) -> HRESULT,
    fn get_nextSibling(
        nextSibling: *mut *mut IXMLDOMNode,
    ) -> HRESULT,
    fn get_attributes(
        attributeMap: *mut *mut IXMLDOMNamedNodeMap,
    ) -> HRESULT,
    fn insertBefore(
        newChild: *mut IXMLDOMNode,
        refChild: VARIANT,
        outNewChild: *mut *mut IXMLDOMNode,
    ) -> HRESULT,
    fn replaceChild(
        newChild: *mut IXMLDOMNode,
        oldChild: *mut IXMLDOMNode,
        outOldChild: *mut *mut IXMLDOMNode,
    ) -> HRESULT,
    fn removeChild(
        childNode: *mut IXMLDOMNode,
        oldChild: *mut *mut IXMLDOMNode,
    ) -> HRESULT,
    fn appendChild(
        newChild: *mut IXMLDOMNode,
        outNewChild: *mut *mut IXMLDOMNode,
    ) -> HRESULT,
    fn hasChildNodes(
        hasChild: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ownerDocument(
        XMLDOMDocument: *mut *mut IXMLDOMDocument,
    ) -> HRESULT,
    fn cloneNode(
        deep: VARIANT_BOOL,
        cloneRoot: *mut *mut IXMLDOMNode,
    ) -> HRESULT,
    fn get_nodeTypeString(
        nodeType: *mut BSTR,
    ) -> HRESULT,
    fn get_text(
        text: *mut BSTR,
    ) -> HRESULT,
    fn put_text(
        text: BSTR,
    ) -> HRESULT,
    fn get_specified(
        isSpecified: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_definition(
        definitionNode: *mut *mut IXMLDOMNode,
    ) -> HRESULT,
    fn get_nodeTypedValue(
        typedValue: *mut VARIANT,
    ) -> HRESULT,
    fn put_nodeTypedValue(
        typedValue: VARIANT,
    ) -> HRESULT,
    fn get_dataType(
        dataTypeName: *mut VARIANT,
    ) -> HRESULT,
    fn put_dataType(
        dataTypeName: BSTR,
    ) -> HRESULT,
    fn get_xml(
        xmlString: *mut BSTR,
    ) -> HRESULT,
    fn transformNode(
        stylesheet: *mut IXMLDOMNode,
        xmlString: *mut BSTR,
    ) -> HRESULT,
    fn selectNodes(
        queryString: BSTR,
        resultList: *mut *mut IXMLDOMNodeList,
    ) -> HRESULT,
    fn selectSingleNode(
        queryString: BSTR,
        resultNode: *mut *mut IXMLDOMNode,
    ) -> HRESULT,
    fn get_parsed(
        isParsed: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_namespaceURI(
        namespaceURI: *mut BSTR,
    ) -> HRESULT,
    fn get_prefix(
        prefixString: *mut BSTR,
    ) -> HRESULT,
    fn get_baseName(
        nameString: *mut BSTR,
    ) -> HRESULT,
    fn transformNodeToObject(
        stylesheet: *mut IXMLDOMNode,
        outputObject: VARIANT,
    ) -> HRESULT,
});
RIDL!(#[uuid(0x3efaa413, 0x272f, 0x11d2, 0x83, 0x6f, 0x00, 0x00, 0xf8, 0x7a, 0x77, 0x82)]
interface IXMLDOMDocumentFragment(IXMLDOMDocumentFragmentVtbl): IXMLDOMNode(IXMLDOMNodeVtbl) {
});
RIDL!(#[uuid(0x2933bf81, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60)]
interface IXMLDOMDocument(IXMLDOMDocumentVtbl): IXMLDOMNode(IXMLDOMNodeVtbl) {
    fn get_doctype(
        documentType: *mut *mut IXMLDOMDocumentType,
    ) -> HRESULT,
    fn get_implementation(
        impl_: *mut *mut IXMLDOMImplementation,
    ) -> HRESULT,
    fn get_documentElement(
        DOMElement: *mut *mut IXMLDOMElement,
    ) -> HRESULT,
    fn putref_documentElement(
        DOMElement: *mut IXMLDOMElement,
    ) -> HRESULT,
    fn createElement(
        tagName: BSTR,
        element: *mut *mut IXMLDOMElement,
    ) -> HRESULT,
    fn createDocumentFragment(
        docFrag: *mut *mut IXMLDOMDocumentFragment,
    ) -> HRESULT,
    fn createTextNode(
        data: BSTR,
        text: *mut *mut IXMLDOMText,
    ) -> HRESULT,
    fn createComment(
        data: BSTR,
        comment: *mut *mut IXMLDOMComment,
    ) -> HRESULT,
    fn createCDATASection(
        data: BSTR,
        cdata: *mut *mut IXMLDOMCDATASection,
    ) -> HRESULT,
    fn createProcessingInstruction(
        target: BSTR,
        data: BSTR,
        pi: *mut *mut IXMLDOMProcessingInstruction,
    ) -> HRESULT,
    fn createAttribute(
        name: BSTR,
        attribute: *mut *mut IXMLDOMAttribute,
    ) -> HRESULT,
    fn createEntityReference(
        name: BSTR,
        entityRef: *mut *mut IXMLDOMEntityReference,
    ) -> HRESULT,
    fn getElementsByTagName(
        tagName: BSTR,
        resultList: *mut *mut IXMLDOMNodeList,
    ) -> HRESULT,
    fn createNode(
        Type: VARIANT,
        name: BSTR,
        namespaceURI: BSTR,
        node: *mut *mut IXMLDOMNode,
    ) -> HRESULT,
    fn nodeFromID(
        idString: BSTR,
        node: *mut *mut IXMLDOMNode,
    ) -> HRESULT,
    fn load(
        xmlSource: VARIANT,
        isSuccessful: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_readyState(
        value: *mut c_long,
    ) -> HRESULT,
    fn get_parseError(
        errorObj: *mut *mut IXMLDOMParseError,
    ) -> HRESULT,
    fn get_url(
        urlString: *mut BSTR,
    ) -> HRESULT,
    fn get_async(
        isAsync: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_async(
        isAsync: VARIANT_BOOL,
    ) -> HRESULT,
    fn abort() -> HRESULT,
    fn loadXML(
        bstrXML: BSTR,
        isSuccessful: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn save(
        destination: VARIANT,
    ) -> HRESULT,
    fn get_validateOnParse(
        isValidating: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_validateOnParse(
        isValidating: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_resolveExternals(
        isResolving: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_resolveExternals(
        isResolving: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_preserveWhiteSpace(
        isPreserving: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_preserveWhiteSpace(
        isPreserving: VARIANT_BOOL,
    ) -> HRESULT,
    fn put_onreadystatechange(
        readystatechangeSink: VARIANT,
    ) -> HRESULT,
    fn put_ondataavailable(
        ondataavailableSink: VARIANT,
    ) -> HRESULT,
    fn put_ontransformnode(
        ontransformnodeSink: VARIANT,
    ) -> HRESULT,
});
RIDL!(#[uuid(0x2933bf82, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60)]
interface IXMLDOMNodeList(IXMLDOMNodeListVtbl): IDispatch(IDispatchVtbl) {
    fn get_item(
        index: c_long,
        listItem: *mut *mut IXMLDOMNode,
    ) -> HRESULT,
    fn get_length(
        listLength: *mut c_long,
    ) -> HRESULT,
    fn nextNode(
        nextItem: *mut *mut IXMLDOMNode,
    ) -> HRESULT,
    fn reset() -> HRESULT,
    fn get__newEnum(
        ppUnk: *mut *mut IUnknown,
    ) -> HRESULT,
});
RIDL!(#[uuid(0x2933bf83, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60)]
interface IXMLDOMNamedNodeMap(IXMLDOMNamedNodeMapVtbl): IDispatch(IDispatchVtbl) {
    fn getNamedItem(
        name: BSTR,
        namedItem: *mut *mut IXMLDOMNode,
    ) -> HRESULT,
    fn setNamedItem(
        newItem: *mut IXMLDOMNode,
        namedItem: *mut *mut IXMLDOMNode,
    ) -> HRESULT,
    fn removeNamedItem(
        name: BSTR,
        namedItem: *mut *mut IXMLDOMNode,
    ) -> HRESULT,
    fn get_item(
        index: c_long,
        listItem: *mut *mut IXMLDOMNode,
    ) -> HRESULT,
    fn get_length(
        listLength: *mut c_long,
    ) -> HRESULT,
    fn getQualifiedItem(
        baseName: BSTR,
        namespaceURI: BSTR,
        qualifiedItem: *mut *mut IXMLDOMNode,
    ) -> HRESULT,
    fn removeQualifiedItem(
        baseName: BSTR,
        namespaceURI: BSTR,
        qualifiedItem: *mut *mut IXMLDOMNode,
    ) -> HRESULT,
    fn nextNode(
        nextItem: *mut *mut IXMLDOMNode,
    ) -> HRESULT,
    fn reset() -> HRESULT,
    fn get__newEnum(
        ppUnk: *mut *mut IUnknown,
    ) -> HRESULT,
});
RIDL!(#[uuid(0x2933bf84, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60)]
interface IXMLDOMCharacterData(IXMLDOMCharacterDataVtbl): IXMLDOMNode(IXMLDOMNodeVtbl) {
    fn get_data(
        data: *mut BSTR,
    ) -> HRESULT,
    fn put_data(
        data: BSTR,
    ) -> HRESULT,
    fn get_length(
        dataLength: *mut c_long,
    ) -> HRESULT,
    fn substringData(
        offset: c_long,
        count: c_long,
        data: *mut BSTR,
    ) -> HRESULT,
    fn appendData(
        data: BSTR,
    ) -> HRESULT,
    fn insertData(
        offset: c_long,
        data: BSTR,
    ) -> HRESULT,
    fn deleteData(
        offset: c_long,
        count: c_long,
    ) -> HRESULT,
    fn replaceData(
        offset: c_long,
        count: c_long,
        data: BSTR,
    ) -> HRESULT,
});
RIDL!(#[uuid(0x2933bf85, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60)]
interface IXMLDOMAttribute(IXMLDOMAttributeVtbl): IXMLDOMNode(IXMLDOMNodeVtbl) {
    fn get_name(
        attributeName: *mut BSTR,
    ) -> HRESULT,
    fn get_value(
        attributeValue: *mut VARIANT,
    ) -> HRESULT,
    fn put_value(
        attributeValue: VARIANT,
    ) -> HRESULT,
});
RIDL!(#[uuid(0x2933bf86, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60)]
interface IXMLDOMElement(IXMLDOMElementVtbl): IXMLDOMNode(IXMLDOMNodeVtbl) {
    fn get_tagName(
        tagName: *mut BSTR,
    ) -> HRESULT,
    fn getAttribute(
        name: BSTR,
        value: *mut VARIANT,
    ) -> HRESULT,
    fn setAttribute(
        name: BSTR,
        value: VARIANT,
    ) -> HRESULT,
    fn removeAttribute(
        name: BSTR,
    ) -> HRESULT,
    fn getAttributeNode(
        name: BSTR,
        attributeNode: *mut *mut IXMLDOMAttribute,
    ) -> HRESULT,
    fn setAttributeNode(
        DOMAttribute: *mut IXMLDOMAttribute,
        attributeNode: *mut *mut IXMLDOMAttribute,
    ) -> HRESULT,
    fn removeAttributeNode(
        DOMAttribute: *mut IXMLDOMAttribute,
        attributeNode: *mut *mut IXMLDOMAttribute,
    ) -> HRESULT,
    fn getElementsByTagName(
        tagName: BSTR,
        resultList: *mut *mut IXMLDOMNodeList,
    ) -> HRESULT,
    fn normalize() -> HRESULT,
});
RIDL!(#[uuid(0x2933bf87, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60)]
interface IXMLDOMText(IXMLDOMTextVtbl): IXMLDOMCharacterData(IXMLDOMCharacterDataVtbl) {
    fn splitText(
        offset: c_long,
        rightHandTextNode: *mut *mut IXMLDOMText,
    ) -> HRESULT,
});
RIDL!(#[uuid(0x2933bf88, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60)]
interface IXMLDOMComment(IXMLDOMCommentVtbl): IXMLDOMCharacterData(IXMLDOMCharacterDataVtbl) {
});
RIDL!(#[uuid(0x2933bf89, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60)]
interface IXMLDOMProcessingInstruction(IXMLDOMProcessingInstructionVtbl):
    IXMLDOMNode(IXMLDOMNodeVtbl) {
    fn get_target(
        name: *mut BSTR,
    ) -> HRESULT,
    fn get_data(
        value: *mut BSTR,
    ) -> HRESULT,
    fn put_data(
        value: BSTR,
    ) -> HRESULT,
});
RIDL!(#[uuid(0x2933bf8a, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60)]
interface IXMLDOMCDATASection(IXMLDOMCDATASectionVtbl): IXMLDOMText(IXMLDOMTextVtbl) {
});
RIDL!(#[uuid(0x2933bf8b, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60)]
interface IXMLDOMDocumentType(IXMLDOMDocumentTypeVtbl): IXMLDOMNode(IXMLDOMNodeVtbl) {
    fn get_name(
        rootName: *mut BSTR,
    ) -> HRESULT,
    fn get_entities(
        entityMap: *mut *mut IXMLDOMNamedNodeMap,
    ) -> HRESULT,
    fn get_notations(
        notationMap: *mut *mut IXMLDOMNamedNodeMap,
    ) -> HRESULT,
});
RIDL!(#[uuid(0x2933bf8c, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60)]
interface IXMLDOMNotation(IXMLDOMNotationVtbl): IXMLDOMNode(IXMLDOMNodeVtbl) {
    fn get_publicId(
        publicID: *mut VARIANT,
    ) -> HRESULT,
    fn get_systemId(
        systemID: *mut VARIANT,
    ) -> HRESULT,
});
RIDL!(#[uuid(0x2933bf8d, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60)]
interface IXMLDOMEntity(IXMLDOMEntityVtbl): IXMLDOMNode(IXMLDOMNodeVtbl) {
    fn get_publicId(
        publicID: *mut VARIANT,
    ) -> HRESULT,
    fn get_systemId(
        systemID: *mut VARIANT,
    ) -> HRESULT,
    fn get_notationName(
        name: *mut BSTR,
    ) -> HRESULT,
});
RIDL!(#[uuid(0x2933bf8e, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60)]
interface IXMLDOMEntityReference(IXMLDOMEntityReferenceVtbl): IXMLDOMNode(IXMLDOMNodeVtbl) {
});
RIDL!(#[uuid(0x3efaa426, 0x272f, 0x11d2, 0x83, 0x6f, 0x00, 0x00, 0xf8, 0x7a, 0x77, 0x82)]
interface IXMLDOMParseError(IXMLDOMParseErrorVtbl): IDispatch(IDispatchVtbl) {
    fn get_errorCode(
        errorCode: *mut c_long,
    ) -> HRESULT,
    fn get_url(
        urlString: *mut BSTR,
    ) -> HRESULT,
    fn get_reason(
        reasonString: *mut BSTR,
    ) -> HRESULT,
    fn get_srcText(
        sourceString: *mut BSTR,
    ) -> HRESULT,
    fn get_line(
        lineNumber: *mut c_long,
    ) -> HRESULT,
    fn get_linepos(
        linePosition: *mut c_long,
    ) -> HRESULT,
    fn get_filepos(
        filePosition: *mut c_long,
    ) -> HRESULT,
});
RIDL!(#[uuid(0x3efaa425, 0x272f, 0x11d2, 0x83, 0x6f, 0x00, 0x00, 0xf8, 0x7a, 0x77, 0x82)]
interface IXTLRuntime(IXTLRuntimeVtbl): IXMLDOMNode(IXMLDOMNodeVtbl) {
    fn uniqueID(
        pNode: *mut IXMLDOMNode,
        pID: *mut c_long,
    ) -> HRESULT,
    fn depth(
        pNode: *mut IXMLDOMNode,
        pDepth: *mut c_long,
    ) -> HRESULT,
    fn childNumber(
        pNode: *mut IXMLDOMNode,
        pNumber: *mut c_long,
    ) -> HRESULT,
    fn ancestorChildNumber(
        bstrNodeName: BSTR,
        pNode: *mut IXMLDOMNode,
        pNumber: *mut c_long,
    ) -> HRESULT,
    fn absoluteChildNumber(
        pNode: *mut IXMLDOMNode,
        pNumber: *mut c_long,
    ) -> HRESULT,
    fn formatIndex(
        lIndex: c_long,
        bstrFormat: BSTR,
        pbstrFormattedString: *mut BSTR,
    ) -> HRESULT,
    fn formatNumber(
        dblNumber: c_double,
        bstrFormat: BSTR,
        pbstrFormattedString: *mut BSTR,
    ) -> HRESULT,
    fn formatDate(
        varDate: VARIANT,
        bstrFormat: BSTR,
        varDestLocale: VARIANT,
        pbstrFormattedString: *mut BSTR,
    ) -> HRESULT,
    fn formatTime(
        varTime: VARIANT,
        bstrFormat: BSTR,
        varDestLocale: VARIANT,
        pbstrFormattedString: *mut BSTR,
    ) -> HRESULT,
});
RIDL!(#[uuid(0x3efaa427, 0x272f, 0x11d2, 0x83, 0x6f, 0x00, 0x00, 0xf8, 0x7a, 0x77, 0x82)]
interface XMLDOMDocumentEvents(XMLDOMDocumentEventsVtbl): IDispatch(IDispatchVtbl) {
});
RIDL!(#[uuid(0xed8c108d, 0x4349, 0x11d2, 0x91, 0xa4, 0x00, 0xc0, 0x4f, 0x79, 0x69, 0xe8)]
interface IXMLHttpRequest(IXMLHttpRequestVtbl): IDispatch(IDispatchVtbl) {
    fn open(
        bstrMethod: BSTR,
        bstrUrl: BSTR,
        varAsync: VARIANT,
        bstrUser: VARIANT,
        bstrPassword: VARIANT,
    ) -> HRESULT,
    fn setRequestHeader(
        bstrHeader: BSTR,
        bstrValue: BSTR,
    ) -> HRESULT,
    fn getResponseHeader(
        bstrHeader: BSTR,
        bstrValue: *mut BSTR,
    ) -> HRESULT,
    fn getAllResponseHeaders(
        pbstrHeaders: *mut BSTR,
    ) -> HRESULT,
    fn send(
        varBody: VARIANT,
    ) -> HRESULT,
    fn abort() -> HRESULT,
    fn get_status(
        plStatus: *mut c_long,
    ) -> HRESULT,
    fn get_statusText(
        pbstrStatus: *mut BSTR,
    ) -> HRESULT,
    fn get_responseXML(
        ppBody: *mut *mut IDispatch,
    ) -> HRESULT,
    fn get_responseText(
        pbstrBody: *mut BSTR,
    ) -> HRESULT,
    fn get_responseBody(
        pvarBody: *mut VARIANT,
    ) -> HRESULT,
    fn get_responseStream(
        pvarBody: *mut VARIANT,
    ) -> HRESULT,
    fn get_readyState(
        plState: *mut c_long,
    ) -> HRESULT,
    fn put_onreadystatechange(
        pReadyStateSink: *mut IDispatch,
    ) -> HRESULT,
});
RIDL!(#[uuid(0x310afa62, 0x0575, 0x11d2, 0x9c, 0xa9, 0x00, 0x60, 0xb0, 0xec, 0x3d, 0x39)]
interface IXMLDSOControl(IXMLDSOControlVtbl): IDispatch(IDispatchVtbl) {
    fn get_XMLDocument(
        ppDoc: *mut *mut IXMLDOMDocument,
    ) -> HRESULT,
    fn put_XMLDocument(
        ppDoc: *mut IXMLDOMDocument,
    ) -> HRESULT,
    fn get_JavaDSOCompatible(
        fJavaDSOCompatible: *mut BOOL,
    ) -> HRESULT,
    fn put_JavaDSOCompatible(
        fJavaDSOCompatible: BOOL,
    ) -> HRESULT,
    fn get_readyState(
        state: *mut c_long,
    ) -> HRESULT,
});
RIDL!(#[uuid(0x65725580, 0x9b5d, 0x11d0, 0x9b, 0xfe, 0x00, 0xc0, 0x4f, 0xc9, 0x9c, 0x8e)]
interface IXMLElementCollection(IXMLElementCollectionVtbl): IDispatch(IDispatchVtbl) {
    fn put_length(
        v: c_long,
    ) -> HRESULT,
    fn get_length(
        p: *mut c_long,
    ) -> HRESULT,
    fn get__newEnum(
        ppUnk: *mut *mut IUnknown,
    ) -> HRESULT,
    fn item(
        var1: VARIANT,
        var2: VARIANT,
        ppDisp: *mut *mut IDispatch,
    ) -> HRESULT,
});
RIDL!(#[uuid(0xf52e2b61, 0x18a1, 0x11d1, 0xb1, 0x05, 0x00, 0x80, 0x5f, 0x49, 0x91, 0x6b)]
interface IXMLDocument(IXMLDocumentVtbl): IDispatch(IDispatchVtbl) {
    fn get_root(
        p: *mut *mut IXMLElement,
    ) -> HRESULT,
    fn get_fileSize(
        p: *mut BSTR,
    ) -> HRESULT,
    fn get_fileModifiedDate(
        p: *mut BSTR,
    ) -> HRESULT,
    fn get_fileUpdatedDate(
        p: *mut BSTR,
    ) -> HRESULT,
    fn get_URL(
        p: *mut BSTR,
    ) -> HRESULT,
    fn put_URL(
        p: BSTR,
    ) -> HRESULT,
    fn get_mimeType(
        p: *mut BSTR,
    ) -> HRESULT,
    fn get_readyState(
        pl: *mut c_long,
    ) -> HRESULT,
    fn get_charset(
        p: *mut BSTR,
    ) -> HRESULT,
    fn put_charset(
        p: BSTR,
    ) -> HRESULT,
    fn get_version(
        p: *mut BSTR,
    ) -> HRESULT,
    fn get_doctype(
        p: *mut BSTR,
    ) -> HRESULT,
    fn get_dtdURL(
        p: *mut BSTR,
    ) -> HRESULT,
    fn createElement(
        var1: VARIANT,
        var2: VARIANT,
        ppElem: *mut *mut IXMLElement,
    ) -> HRESULT,
});
RIDL!(#[uuid(0x2b8de2fe, 0x8d2d, 0x11d1, 0xb2, 0xfc, 0x00, 0xc0, 0x4f, 0xd9, 0x15, 0xa9)]
interface IXMLDocument2(IXMLDocument2Vtbl): IDispatch(IDispatchVtbl) {
    fn get_root(
        p: *mut *mut IXMLElement2,
    ) -> HRESULT,
    fn get_fileSize(
        p: *mut BSTR,
    ) -> HRESULT,
    fn get_fileModifiedDate(
        p: *mut BSTR,
    ) -> HRESULT,
    fn get_fileUpdatedDate(
        p: *mut BSTR,
    ) -> HRESULT,
    fn get_URL(
        p: *mut BSTR,
    ) -> HRESULT,
    fn put_URL(
        p: BSTR,
    ) -> HRESULT,
    fn get_mimeType(
        p: *mut BSTR,
    ) -> HRESULT,
    fn get_readyState(
        pl: *mut c_long,
    ) -> HRESULT,
    fn get_charset(
        p: *mut BSTR,
    ) -> HRESULT,
    fn put_charset(
        p: BSTR,
    ) -> HRESULT,
    fn get_version(
        p: *mut BSTR,
    ) -> HRESULT,
    fn get_doctype(
        p: *mut BSTR,
    ) -> HRESULT,
    fn get_dtdURL(
        p: *mut BSTR,
    ) -> HRESULT,
    fn createElement(
        var1: VARIANT,
        var2: VARIANT,
        ppElem: *mut *mut IXMLElement,
    ) -> HRESULT,
    fn get_async(
        pf: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_async(
        f: VARIANT_BOOL,
    ) -> HRESULT,
});
RIDL!(#[uuid(0x3f7f31ac, 0xe15f, 0x11d0, 0x9c, 0x25, 0x00, 0xc0, 0x4f, 0xc9, 0x9c, 0x8e)]
interface IXMLElement(IXMLElementVtbl): IDispatch(IDispatchVtbl) {
    fn get_tagName(
        p: *mut BSTR,
    ) -> HRESULT,
    fn put_tagName(
        p: BSTR,
    ) -> HRESULT,
    fn get_parent(
        ppParent: *mut *mut IXMLElement,
    ) -> HRESULT,
    fn setAttribute(
        strPropertyName: BSTR,
        PropertyValue: VARIANT,
    ) -> HRESULT,
    fn getAttribute(
        strPropertyName: BSTR,
        PropertyValue: *mut VARIANT,
    ) -> HRESULT,
    fn removeAttribute(
        strPropertyName: BSTR,
    ) -> HRESULT,
    fn get_children(
        pp: *mut *mut IXMLElementCollection,
    ) -> HRESULT,
    fn get_type(
        plType: *mut c_long,
    ) -> HRESULT,
    fn get_text(
        p: *mut BSTR,
    ) -> HRESULT,
    fn put_text(
        p: BSTR,
    ) -> HRESULT,
    fn addChild(
        pChildElem: *mut IXMLElement,
        lIndex: c_long,
        lReserved: c_long,
    ) -> HRESULT,
    fn removeChild(
        pChildElem: *mut IXMLElement,
    ) -> HRESULT,
});
RIDL!(#[uuid(0x2b8de2ff, 0x8d2d, 0x11d1, 0xb2, 0xfc, 0x00, 0xc0, 0x4f, 0xd9, 0x15, 0xa9)]
interface IXMLElement2(IXMLElement2Vtbl): IDispatch(IDispatchVtbl) {
    fn get_tagName(
        p: *mut BSTR,
    ) -> HRESULT,
    fn put_tagName(
        p: BSTR,
    ) -> HRESULT,
    fn get_parent(
        ppParent: *mut *mut IXMLElement,
    ) -> HRESULT,
    fn setAttribute(
        strPropertyName: BSTR,
        PropertyValue: VARIANT,
    ) -> HRESULT,
    fn getAttribute(
        strPropertyName: BSTR,
        PropertyValue: *mut VARIANT,
    ) -> HRESULT,
    fn removeAttribute(
        strPropertyName: BSTR,
    ) -> HRESULT,
    fn get_children(
        pp: *mut *mut IXMLElementCollection,
    ) -> HRESULT,
    fn get_type(
        plType: *mut c_long,
    ) -> HRESULT,
    fn get_text(
        p: *mut BSTR,
    ) -> HRESULT,
    fn put_text(
        p: BSTR,
    ) -> HRESULT,
    fn addChild(
        pChildElem: *mut IXMLElement,
        lIndex: c_long,
        lReserved: c_long,
    ) -> HRESULT,
    fn removeChild(
        pChildElem: *mut IXMLElement,
    ) -> HRESULT,
    fn get_attributes(
        pp: *mut *mut IXMLElementCollection,
    ) -> HRESULT,
});
RIDL!(#[uuid(0xd4d4a0fc, 0x3b73, 0x11d1, 0xb2, 0xb4, 0x00, 0xc0, 0x4f, 0xb9, 0x25, 0x96)]
interface IXMLAttribute(IXMLAttributeVtbl): IDispatch(IDispatchVtbl) {
    fn get_name(
        n: *mut BSTR,
    ) -> HRESULT,
    fn get_value(
        v: *mut BSTR,
    ) -> HRESULT,
});
RIDL!(#[uuid(0x948c5ad3, 0xc58d, 0x11d0, 0x9c, 0x0b, 0x00, 0xc0, 0x4f, 0xc9, 0x9c, 0x8e)]
interface IXMLError(IXMLErrorVtbl): IUnknown(IUnknownVtbl) {
    fn GetErrorInfo(
        pErrorReturn: *mut XML_ERROR,
    ) -> HRESULT,
});
