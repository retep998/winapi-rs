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
use um::winnt::{HRESULT};
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
// extern RPC_IF_HANDLE __MIDL_itf_msxml_0000_0000_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_msxml_0000_0000_v0_0_s_ifspec;
ENUM!{enum DOMNodeType {
    NODE_INVALID,
    NODE_ELEMENT,
    NODE_ATTRIBUTE,
    NODE_TEXT,
    NODE_CDATA_SECTION,
    NODE_ENTITY_REFERENCE,
    NODE_ENTITY,
    NODE_PROCESSING_INSTRUCTION,
    NODE_COMMENT,
    NODE_DOCUMENT,
    NODE_DOCUMENT_TYPE,
    NODE_DOCUMENT_FRAGMENT,
    NODE_NOTATION,
}}
ENUM!{enum XMLELEM_TYPE {
    XMLELEMTYPE_ELEMENT,
    XMLELEMTYPE_TEXT,
    XMLELEMTYPE_COMMENT,
    XMLELEMTYPE_DOCUMENT,
    XMLELEMTYPE_DTD,
    XMLELEMTYPE_PI,
    XMLELEMTYPE_OTHER,
}}
// EXTERN_C const IID LIBID_MSXML;
DEFINE_GUID!{IID_IXMLDOMImplementation,
    0x2933bf8f, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60}
RIDL!{#[uuid(0x2933bf8f, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60)]
interface IXMLDOMImplementation(IXMLDOMImplementationVtbl): IDispatch(IDispatchVtbl) {
    fn hasFeature(
        feature: BSTR,
        version: BSTR,
        hasFeature: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IXMLDOMNode,
    0x2933bf80, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60}
RIDL!{#[uuid(0x2933bf80, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60)]
interface IXMLDOMNode(IXMLDOMNodeVtbl): IDispatch(IDispatchVtbl) {
    fn nodeName(
        name: *mut BSTR,
    ) -> HRESULT,
    fn nodeValue_2(
        value: VARIANT,
    ) -> HRESULT,
    fn nodeValue_1(
        value: *mut VARIANT,
    ) -> HRESULT,
    fn nodeType(
        type_: *mut DOMNodeType,
    ) -> HRESULT,
    fn parentNode(
        parent: *mut *mut IXMLDOMNode,
    ) -> HRESULT,
    fn childNodes(
        childList: *mut *mut IXMLDOMNodeList,
    ) -> HRESULT,
    fn firstChild(
        firstChild: *mut *mut IXMLDOMNode,
    ) -> HRESULT,
    fn lastChild(
        lastChild: *mut *mut IXMLDOMNode,
    ) -> HRESULT,
    fn previousSibling(
        previousSibling: *mut *mut IXMLDOMNode,
    ) -> HRESULT,
    fn nextSibling(
        nextSibling: *mut *mut IXMLDOMNode,
    ) -> HRESULT,
    fn attributes(
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
    fn ownerDocument(
        DOMDocument: *mut *mut IXMLDOMDocument,
    ) -> HRESULT,
    fn cloneNode(
        deep: VARIANT_BOOL,
        cloneRoot: *mut *mut IXMLDOMNode,
    ) -> HRESULT,
    fn nodeTypeString(
        nodeType: *mut BSTR,
    ) -> HRESULT,
    fn text_2(
        text: BSTR,
    ) -> HRESULT,
    fn text_1(
        text: *mut BSTR,
    ) -> HRESULT,
    fn specified(
        isSpecified: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn definition(
        definitionNode: *mut *mut IXMLDOMNode,
    ) -> HRESULT,
    fn nodeTypedValue_2(
        typedValue: VARIANT,
    ) -> HRESULT,
    fn nodeTypedValue_1(
        typedValue: *mut VARIANT,
    ) -> HRESULT,
    fn dataType_2(
        dataTypeName: BSTR,
    ) -> HRESULT,
    fn dataType_1(
        dataTypeName: *mut VARIANT,
    ) -> HRESULT,
    fn xml(
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
    fn parsed(
        isParsed: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn namespaceURI(
        namespaceURI: *mut BSTR,
    ) -> HRESULT,
    fn prefix(
        prefixString: *mut BSTR,
    ) -> HRESULT,
    fn baseName(
        nameString: *mut BSTR,
    ) -> HRESULT,
    fn transformNodeToObject(
        stylesheet: *mut IXMLDOMNode,
        outputObject: VARIANT,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IXMLDOMDocumentFragment,
    0x3efaa413, 0x272f, 0x11d2, 0x83, 0x6f, 0x00, 0x00, 0xf8, 0x7a, 0x77, 0x82}
RIDL!{#[uuid(0x3efaa413, 0x272f, 0x11d2, 0x83, 0x6f, 0x00, 0x00, 0xf8, 0x7a, 0x77, 0x82)]
interface IXMLDOMDocumentFragment(IXMLDOMDocumentFragmentVtbl): IXMLDOMNode(IXMLDOMNodeVtbl) {
}}
DEFINE_GUID!{IID_IXMLDOMDocument,
    0x2933bf81, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60}
RIDL!{#[uuid(0x2933bf81, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60)]
interface IXMLDOMDocument(IXMLDOMDocumentVtbl): IXMLDOMNode(IXMLDOMNodeVtbl) {
    fn doctype(
        documentType: *mut *mut IXMLDOMDocumentType,
    ) -> HRESULT,
    fn implementation(
        impl_: *mut *mut IXMLDOMImplementation,
    ) -> HRESULT,
    fn documentElement_2(
        DOMElement: *mut IXMLDOMElement,
    ) -> HRESULT,
    fn documentElement_1(
        DOMElement: *mut *mut IXMLDOMElement,
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
    fn readyState(
        value: *mut c_long,
    ) -> HRESULT,
    fn parseError(
        errorObj: *mut *mut IXMLDOMParseError,
    ) -> HRESULT,
    fn url(
        urlString: *mut BSTR,
    ) -> HRESULT,
    fn async_2(
        isAsync: VARIANT_BOOL,
    ) -> HRESULT,
    fn async_1(
        isAsync: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn abort() -> HRESULT,
    fn loadXML(
        bstrXML: BSTR,
        isSuccessful: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn save(
        desination: VARIANT,
    ) -> HRESULT,
    fn validateOnParse_2(
        isValidating: VARIANT_BOOL,
    ) -> HRESULT,
    fn validateOnParse_1(
        isValidating: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn resolveExternals_2(
        isResolving: VARIANT_BOOL,
    ) -> HRESULT,
    fn resolveExternals_1(
        isResolving: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn preserveWhiteSpace_2(
        isPreserving: VARIANT_BOOL,
    ) -> HRESULT,
    fn preserveWhiteSpace_1(
        isPreserving: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn onreadystatechange(
        readystatechangeSink: VARIANT,
    ) -> HRESULT,
    fn ondataavailable(
        ondataavailableSink: VARIANT,
    ) -> HRESULT,
    fn ontransformnode(
        ontransformnodeSink: VARIANT,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IXMLDOMNodeList,
    0x2933bf82, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60}
RIDL!{#[uuid(0x2933bf82, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60)]
interface IXMLDOMNodeList(IXMLDOMNodeListVtbl): IDispatch(IDispatchVtbl) {
    fn item(
        index: c_long,
        listItem: *mut *mut IXMLDOMNode,
    ) -> HRESULT,
    fn length(
        listLength: *mut c_long,
    ) -> HRESULT,
    fn nextNode(
        nextItem: *mut *mut IXMLDOMNode,
    ) -> HRESULT,
    fn reset() -> HRESULT,
    fn _newEnum(
        ppUnk: *mut *mut IUnknown,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IXMLDOMNamedNodeMap,
    0x2933bf83, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60}
RIDL!{#[uuid(0x2933bf83, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60)]
interface IXMLDOMNamedNodeMap(IXMLDOMNamedNodeMapVtbl): IDispatch(IDispatchVtbl) {
    fn getNamedItem(
        name: BSTR,
        namedItem: *mut *mut IXMLDOMNode,
    ) -> HRESULT,
    fn setNamedItem(
        newItem: *mut IXMLDOMNode,
        nameItem: *mut *mut IXMLDOMNode,
    ) -> HRESULT,
    fn removeNamedItem(
        name: BSTR,
        namedItem: *mut *mut IXMLDOMNode,
    ) -> HRESULT,
    fn item(
        index: c_long,
        listItem: *mut *mut IXMLDOMNode,
    ) -> HRESULT,
    fn length(
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
    fn _newEnum(
        ppUnk: *mut *mut IUnknown,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IXMLDOMCharacterData,
    0x2933bf84, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60}
RIDL!{#[uuid(0x2933bf84, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60)]
interface IXMLDOMCharacterData(IXMLDOMCharacterDataVtbl): IXMLDOMNode(IXMLDOMNodeVtbl) {
    fn data_2(
        data: BSTR,
    ) -> HRESULT,
    fn data_1(
        data: *mut BSTR,
    ) -> HRESULT,
    fn length(
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
}}
DEFINE_GUID!{IID_IXMLDOMAttribute,
    0x2933bf85, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60}
RIDL!{#[uuid(0x2933bf85, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60)]
interface IXMLDOMAttribute(IXMLDOMAttributeVtbl): IXMLDOMNode(IXMLDOMNodeVtbl) {
    fn name(
        attributeName: *mut BSTR,
    ) -> HRESULT,
    fn value_2(
        attributeValue: VARIANT,
    ) -> HRESULT,
    fn value_1(
        attributeValue: *mut VARIANT,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IXMLDOMElement,
    0x2933bf86, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60}
RIDL!{#[uuid(0x2933bf86, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60)]
interface IXMLDOMElement(IXMLDOMElementVtbl): IXMLDOMNode(IXMLDOMNodeVtbl) {
    fn tagName(
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
}}
DEFINE_GUID!{IID_IXMLDOMText,
    0x2933bf87, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60}
RIDL!{#[uuid(0x2933bf87, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60)]
interface IXMLDOMText(IXMLDOMTextVtbl): IXMLDOMCharacterData(IXMLDOMCharacterDataVtbl) {
    fn splitText(
        offset: c_long,
        rightHandTextNode: *mut *mut IXMLDOMText,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IXMLDOMComment,
    0x2933bf88, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60}
RIDL!{#[uuid(0x2933bf88, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60)]
interface IXMLDOMComment(IXMLDOMCommentVtbl): IXMLDOMCharacterData(IXMLDOMCharacterDataVtbl) {
}}
DEFINE_GUID!{IID_IXMLDOMProcessingInstruction,
    0x2933bf89, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60}
RIDL!{#[uuid(0x2933bf89, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60)]
interface IXMLDOMProcessingInstruction(IXMLDOMProcessingInstructionVtbl): IXMLDOMNode(IXMLDOMNodeVtbl) {
    fn target(
        name: *mut BSTR,
    ) -> HRESULT,
    fn data_2(
        value: BSTR,
    ) -> HRESULT,
    fn data_1(
        value: *mut BSTR,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IXMLDOMCDATASection,
    0x2933bf8a, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60}
RIDL!{#[uuid(0x2933bf8a, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60)]
interface IXMLDOMCDATASection(IXMLDOMCDATASectionVtbl): IXMLDOMText(IXMLDOMTextVtbl) {
}}
DEFINE_GUID!{IID_IXMLDOMDocumentType,
    0x2933bf8b, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60}
RIDL!{#[uuid(0x2933bf8b, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60)]
interface IXMLDOMDocumentType(IXMLDOMDocumentTypeVtbl): IXMLDOMNode(IXMLDOMNodeVtbl) {
    fn name(
        rootName: *mut BSTR,
    ) -> HRESULT,
    fn entities(
        entityMap: *mut *mut IXMLDOMNamedNodeMap,
    ) -> HRESULT,
    fn notations(
        notationMap: *mut *mut IXMLDOMNamedNodeMap,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IXMLDOMNotation,
    0x2933bf8c, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60}
RIDL!{#[uuid(0x2933bf8c, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60)]
interface IXMLDOMNotation(IXMLDOMNotationVtbl): IXMLDOMNode(IXMLDOMNodeVtbl) {
    fn publicId(
        publicID: *mut VARIANT,
    ) -> HRESULT,
    fn systemId(
        systemID: *mut VARIANT,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IXMLDOMEntity,
    0x2933bf8d, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60}
RIDL!{#[uuid(0x2933bf8d, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60)]
interface IXMLDOMEntity(IXMLDOMEntityVtbl): IXMLDOMNode(IXMLDOMNodeVtbl) {
    fn publicId(
        publicID: *mut VARIANT,
    ) -> HRESULT,
    fn systemId(
        systemID: *mut VARIANT,
    ) -> HRESULT,
    fn notationName(
        name: *mut BSTR,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IXMLDOMEntityReference,
    0x2933bf8e, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60}
RIDL!{#[uuid(0x2933bf8e, 0x7b36, 0x11d2, 0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60)]
interface IXMLDOMEntityReference(IXMLDOMEntityReferenceVtbl): IXMLDOMNode(IXMLDOMNodeVtbl) {
}}
DEFINE_GUID!{IID_IXMLDOMParseError,
    0x3efaa426, 0x272f, 0x11d2, 0x83, 0x6f, 0x00, 0x00, 0xf8, 0x7a, 0x77, 0x82}
RIDL!{#[uuid(0x3efaa426, 0x272f, 0x11d2, 0x83, 0x6f, 0x00, 0x00, 0xf8, 0x7a, 0x77, 0x82)]
interface IXMLDOMParseError(IXMLDOMParseErrorVtbl): IDispatch(IDispatchVtbl) {
    fn errorCode(
        errorCode: *mut c_long,
    ) -> HRESULT,
    fn url(
        urlString: *mut BSTR,
    ) -> HRESULT,
    fn reason(
        reasonString: *mut BSTR,
    ) -> HRESULT,
    fn srcText(
        sourceString: *mut BSTR,
    ) -> HRESULT,
    fn line(
        lineNumber: *mut c_long,
    ) -> HRESULT,
    fn linepos(
        linePosition: *mut c_long,
    ) -> HRESULT,
    fn filepos(
        filePosition: *mut c_long,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IXTLRuntime,
    0x3efaa425, 0x272f, 0x11d2, 0x83, 0x6f, 0x00, 0x00, 0xf8, 0x7a, 0x77, 0x82}
RIDL!{#[uuid(0x3efaa425, 0x272f, 0x11d2, 0x83, 0x6f, 0x00, 0x00, 0xf8, 0x7a, 0x77, 0x82)]
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
}}
DEFINE_GUID!{DIID_XMLDOMDocumentEvents,
    0x3efaa427, 0x272f, 0x11d2, 0x83, 0x6f, 0x00, 0x00, 0xf8, 0x7a, 0x77, 0x82}
RIDL!{#[uuid(0x3efaa427, 0x272f, 0x11d2, 0x83, 0x6f, 0x00, 0x00, 0xf8, 0x7a, 0x77, 0x82)]
interface XMLDOMDocumentEvents(XMLDOMDocumentEventsVtbl): IDispatch(IDispatchVtbl) {
}}
DEFINE_GUID!{CLSID_DOMDocument,
    0x2933BF90, 0x7B36, 0x11d2, 0xB2, 0x0E, 0x00, 0xC0, 0x4F, 0x98, 0x3E, 0x60}
// class DECLSPEC_UUID("2933BF90-7B36-11d2-B20E-00C04F983E60")
// DOMDocument;
DEFINE_GUID!{CLSID_DOMFreeThreadedDocument,
    0x2933BF91, 0x7B36, 0x11d2, 0xB2, 0x0E, 0x00, 0xC0, 0x4F, 0x98, 0x3E, 0x60}
// class DECLSPEC_UUID("2933BF91-7B36-11d2-B20E-00C04F983E60")
// DOMFreeThreadedDocument;
DEFINE_GUID!{IID_IXMLHttpRequest,
    0xed8c108d, 0x4349, 0x11d2, 0x91, 0xa4, 0x00, 0xc0, 0x4f, 0x79, 0x69, 0xe8}
RIDL!{#[uuid(0xed8c108d, 0x4349, 0x11d2, 0x91, 0xa4, 0x00, 0xc0, 0x4f, 0x79, 0x69, 0xe8)]
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
        pbstrValue: *mut BSTR,
    ) -> HRESULT,
    fn getAllResponseHeaders(
        pbstrHeaders: *mut BSTR,
    ) -> HRESULT,
    fn send(
        varBody: VARIANT,
    ) -> HRESULT,
    fn abort() -> HRESULT,
    fn status(
        plStatus: *mut c_long,
    ) -> HRESULT,
    fn statusText(
        pbstrStatus: *mut BSTR,
    ) -> HRESULT,
    fn responseXML(
        ppBody: *mut *mut IDispatch,
    ) -> HRESULT,
    fn responseText(
        pbstrBody: *mut BSTR,
    ) -> HRESULT,
    fn responseBody(
        pvarBody: *mut VARIANT,
    ) -> HRESULT,
    fn responseStream(
        pvarBody: *mut VARIANT,
    ) -> HRESULT,
    fn readyState(
        plState: *mut c_long,
    ) -> HRESULT,
    fn onreadystatechange(
        pReadyStateSink: *mut IDispatch,
    ) -> HRESULT,
}}
DEFINE_GUID!{CLSID_XMLHTTPRequest,
    0xED8C108E, 0x4349, 0x11D2, 0x91, 0xA4, 0x00, 0xC0, 0x4F, 0x79, 0x69, 0xE8}
// class DECLSPEC_UUID("ED8C108E-4349-11D2-91A4-00C04F7969E8")
// XMLHTTPRequest;
DEFINE_GUID!{IID_IXMLDSOControl,
    0x310afa62, 0x0575, 0x11d2, 0x9c, 0xa9, 0x00, 0x60, 0xb0, 0xec, 0x3d, 0x39}
RIDL!{#[uuid(0x310afa62, 0x0575, 0x11d2, 0x9c, 0xa9, 0x00, 0x60, 0xb0, 0xec, 0x3d, 0x39)]
interface IXMLDSOControl(IXMLDSOControlVtbl): IDispatch(IDispatchVtbl) {
    fn XMLDocument_2(
        ppDoc: *mut IXMLDOMDocument,
    ) -> HRESULT,
    fn XMLDocument_1(
        ppDoc: *mut *mut IXMLDOMDocument,
    ) -> HRESULT,
    fn JavaDSOCompatible_2(
        fJavaDSOCompatible: BOOL,
    ) -> HRESULT,
    fn JavaDSOCompatible_1(
        fJavaDSOCompatible: *mut BOOL,
    ) -> HRESULT,
    fn readyState(
        state: *mut c_long,
    ) -> HRESULT,
}}
DEFINE_GUID!{CLSID_XMLDSOControl,
    0x550dda30, 0x0541, 0x11d2, 0x9c, 0xa9, 0x00, 0x60, 0xb0, 0xec, 0x3d, 0x39}
// class DECLSPEC_UUID("550dda30-0541-11d2-9ca9-0060b0ec3d39")
// XMLDSOControl;
DEFINE_GUID!{IID_IXMLElementCollection,
    0x65725580, 0x9b5d, 0x11d0, 0x9b, 0xfe, 0x00, 0xc0, 0x4f, 0xc9, 0x9c, 0x8e}
RIDL!{#[uuid(0x65725580, 0x9b5d, 0x11d0, 0x9b, 0xfe, 0x00, 0xc0, 0x4f, 0xc9, 0x9c, 0x8e)]
interface IXMLElementCollection(IXMLElementCollectionVtbl): IDispatch(IDispatchVtbl) {
    fn length_2(
        p: *mut c_long,
    ) -> HRESULT,
    fn length_1(
        v: c_long,
    ) -> HRESULT,
    fn _newEnum(
        ppUnk: *mut *mut IUnknown,
    ) -> HRESULT,
    fn item(
        var1: VARIANT,
        var2: VARIANT,
        ppDisp: *mut *mut IDispatch,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IXMLDocument,
    0xf52e2b61, 0x18a1, 0x11d1, 0xb1, 0x05, 0x00, 0x80, 0x5f, 0x49, 0x91, 0x6b}
RIDL!{#[uuid(0xf52e2b61, 0x18a1, 0x11d1, 0xb1, 0x05, 0x00, 0x80, 0x5f, 0x49, 0x91, 0x6b)]
interface IXMLDocument(IXMLDocumentVtbl): IDispatch(IDispatchVtbl) {
    fn root(
        p: *mut *mut IXMLElement,
    ) -> HRESULT,
    fn fileSize(
        p: *mut BSTR,
    ) -> HRESULT,
    fn fileModifiedDate(
        p: *mut BSTR,
    ) -> HRESULT,
    fn fileUpdatedDate(
        p: *mut BSTR,
    ) -> HRESULT,
    fn URL_2(
        p: BSTR,
    ) -> HRESULT,
    fn URL_1(
        p: *mut BSTR,
    ) -> HRESULT,
    fn mimeType(
        p: *mut BSTR,
    ) -> HRESULT,
    fn readyState(
        pl: *mut c_long,
    ) -> HRESULT,
    fn charset_2(
        p: BSTR,
    ) -> HRESULT,
    fn charset_1(
        p: *mut BSTR,
    ) -> HRESULT,
    fn version(
        p: *mut BSTR,
    ) -> HRESULT,
    fn doctype(
        p: *mut BSTR,
    ) -> HRESULT,
    fn dtdURL(
        p: *mut BSTR,
    ) -> HRESULT,
    fn createElement(
        vType: VARIANT,
        var1: VARIANT,
        ppElem: *mut *mut IXMLElement,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IXMLDocument2,
    0x2b8de2fe, 0x8d2d, 0x11d1, 0xb2, 0xfc, 0x00, 0xc0, 0x4f, 0xd9, 0x15, 0xa9}
RIDL!{#[uuid(0x2b8de2fe, 0x8d2d, 0x11d1, 0xb2, 0xfc, 0x00, 0xc0, 0x4f, 0xd9, 0x15, 0xa9)]
interface IXMLDocument2(IXMLDocument2Vtbl): IDispatch(IDispatchVtbl) {
    fn root(
        p: *mut *mut IXMLElement2,
    ) -> HRESULT,
    fn fileSize(
        p: *mut BSTR,
    ) -> HRESULT,
    fn fileModifiedDate(
        p: *mut BSTR,
    ) -> HRESULT,
    fn fileUpdatedDate(
        p: *mut BSTR,
    ) -> HRESULT,
    fn URL_2(
        p: BSTR,
    ) -> HRESULT,
    fn URL_1(
        p: *mut BSTR,
    ) -> HRESULT,
    fn mimeType(
        p: *mut BSTR,
    ) -> HRESULT,
    fn readyState(
        pl: *mut c_long,
    ) -> HRESULT,
    fn charset_2(
        p: BSTR,
    ) -> HRESULT,
    fn charset_1(
        p: *mut BSTR,
    ) -> HRESULT,
    fn version(
        p: *mut BSTR,
    ) -> HRESULT,
    fn doctype(
        p: *mut BSTR,
    ) -> HRESULT,
    fn dtdURL(
        p: *mut BSTR,
    ) -> HRESULT,
    fn createElement(
        vType: VARIANT,
        var1: VARIANT,
        ppElem: *mut *mut IXMLElement2,
    ) -> HRESULT,
    fn async_2(
        f: VARIANT_BOOL,
    ) -> HRESULT,
    fn async_1(
        pf: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IXMLElement,
    0x3f7f31ac, 0xe15f, 0x11d0, 0x9c, 0x25, 0x00, 0xc0, 0x4f, 0xc9, 0x9c, 0x8e}
RIDL!{#[uuid(0x3f7f31ac, 0xe15f, 0x11d0, 0x9c, 0x25, 0x00, 0xc0, 0x4f, 0xc9, 0x9c, 0x8e)]
interface IXMLElement(IXMLElementVtbl): IDispatch(IDispatchVtbl) {
    fn tagName_2(
        p: BSTR,
    ) -> HRESULT,
    fn tagName_1(
        p: *mut BSTR,
    ) -> HRESULT,
    fn parent_(
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
    fn children(
        pp: *mut *mut IXMLElementCollection,
    ) -> HRESULT,
    fn type_(
        plType: *mut c_long,
    ) -> HRESULT,
    fn text_2(
        p: BSTR,
    ) -> HRESULT,
    fn text_1(
        p: *mut BSTR,
    ) -> HRESULT,
    fn addChild(
        pChildElem: *mut IXMLElement,
        lIndex: c_long,
        lReserved: c_long,
    ) -> HRESULT,
    fn removeChild(
        pChildElem: *mut IXMLElement,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IXMLElement2,
    0x2b8de2ff, 0x8d2d, 0x11d1, 0xb2, 0xfc, 0x00, 0xc0, 0x4f, 0xd9, 0x15, 0xa9}
RIDL!{#[uuid(0x2b8de2ff, 0x8d2d, 0x11d1, 0xb2, 0xfc, 0x00, 0xc0, 0x4f, 0xd9, 0x15, 0xa9)]
interface IXMLElement2(IXMLElement2Vtbl): IDispatch(IDispatchVtbl) {
    fn tagName_2(
        p: BSTR,
    ) -> HRESULT,
    fn tagName_1(
        p: *mut BSTR,
    ) -> HRESULT,
    fn parent_(
        ppParent: *mut *mut IXMLElement2,
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
    fn children(
        pp: *mut *mut IXMLElementCollection,
    ) -> HRESULT,
    fn type_(
        plType: *mut c_long,
    ) -> HRESULT,
    fn text_2(
        p: BSTR,
    ) -> HRESULT,
    fn text_1(
        p: *mut BSTR,
    ) -> HRESULT,
    fn addChild(
        pChildElem: *mut IXMLElement2,
        lIndex: c_long,
        lReserved: c_long,
    ) -> HRESULT,
    fn removeChild(
        pChildElem: *mut IXMLElement2,
    ) -> HRESULT,
    fn attributes(
        pp: *mut *mut IXMLElementCollection,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IXMLAttribute,
    0xd4d4a0fc, 0x3b73, 0x11d1, 0xb2, 0xb4, 0x00, 0xc0, 0x4f, 0xb9, 0x25, 0x96}
RIDL!{#[uuid(0xd4d4a0fc, 0x3b73, 0x11d1, 0xb2, 0xb4, 0x00, 0xc0, 0x4f, 0xb9, 0x25, 0x96)]
interface IXMLAttribute(IXMLAttributeVtbl): IDispatch(IDispatchVtbl) {
    fn name(
        n: *mut BSTR,
    ) -> HRESULT,
    fn value(
        v: *mut BSTR,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IXMLError,
    0x948c5ad3, 0xc58d, 0x11d0, 0x9c, 0x0b, 0x00, 0xc0, 0x4f, 0xc9, 0x9c, 0x8e}
RIDL!{#[uuid(0x948c5ad3, 0xc58d, 0x11d0, 0x9c, 0x0b, 0x00, 0xc0, 0x4f, 0xc9, 0x9c, 0x8e)]
interface IXMLError(IXMLErrorVtbl): IUnknown(IUnknownVtbl) {
    fn GetErrorInfo(
        pErrorReturn: *mut XML_ERROR,
    ) -> HRESULT,
}}
DEFINE_GUID!{CLSID_XMLDocument,
    0xCFC399AF, 0xD876, 0x11d0, 0x9C, 0x10, 0x00, 0xC0, 0x4F, 0xC9, 0x9C, 0x8E}
// class DECLSPEC_UUID("CFC399AF-D876-11d0-9C10-00C04FC99C8E")
// XMLDocument;
// extern RPC_IF_HANDLE __MIDL_itf_msxml_0000_0001_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_msxml_0000_0001_v0_0_s_ifspec;
