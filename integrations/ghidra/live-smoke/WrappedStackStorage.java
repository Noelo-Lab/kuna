// Decode the KUNA_WRAPPED_WIRE_DUMP output from wrapped_stack_tests in stock Ghidra.
import ghidra.app.script.GhidraScript;
import ghidra.program.model.address.*;
import ghidra.program.model.listing.VariableStorage;
import ghidra.program.model.pcode.*;
import java.io.ByteArrayOutputStream;
import java.io.File;
import java.util.HashMap;
import javax.xml.parsers.DocumentBuilderFactory;
import org.w3c.dom.*;
import static ghidra.program.model.pcode.ElementId.*;

public class WrappedStackStorage extends GhidraScript {
    @Override
    protected void run() throws Exception {
        Document document = DocumentBuilderFactory.newInstance().newDocumentBuilder()
            .parse(new File(getScriptArgs()[0]));
        PcodeSyntaxTree factory = new PcodeSyntaxTree(currentProgram.getAddressFactory(),
            new PcodeDataTypeManager(currentProgram, null));
        HashMap<String, VariableStorage> storage = new HashMap<>();
        NodeList symbols = document.getElementsByTagName("mapsym");
        for (int i = 0; i < symbols.getLength(); i++) {
            Element symbol = (Element) symbols.item(i);
            String name = ((Element) symbol.getElementsByTagName("symbol").item(0))
                .getAttribute("name");
            Element address = (Element) symbol.getElementsByTagName("addr").item(0);
            ByteArrayOutputStream buffer = new ByteArrayOutputStream();
            PackedEncode encode = new PackedEncode(buffer);
            if (address.getAttribute("space").equals("join")) {
                Varnode[] pieces = new Varnode[2];
                for (int j = 0; j < pieces.length; j++) {
                    String[] piece = address.getAttribute("piece" + (j + 1)).split(":");
                    AddressSpace space = currentProgram.getAddressFactory().getAddressSpace(piece[0]);
                    pieces[j] = new Varnode(space.getAddress(Long.decode(piece[1])), Integer.parseInt(piece[2]));
                }
                AddressXML.encode(encode, pieces, 0);
            } else {
                AddressSpace space = currentProgram.getAddressFactory().getAddressSpace(address.getAttribute("space"));
                AddressXML.encode(encode, space.getAddress(Long.decode(address.getAttribute("offset"))));
            }
            byte[] bytes = buffer.toByteArray();
            PackedDecode decode = new PackedDecode(currentProgram.getAddressFactory());
            decode.open(bytes.length, "wrapped-stack-storage");
            decode.ingestBytes(bytes, 0, bytes.length);
            decode.endIngest();
            int element = decode.openElement(ELEM_ADDR);
            storage.put(name, AddressXML.decodeStorageFromAttributes(72, decode, factory));
            decode.closeElement(element);
        }
        if (storage.size() != 3 || storage.get("first").equals(storage.get("second")) ||
            storage.get("first").equals(storage.get("ordinary")) ||
            storage.get("second").equals(storage.get("ordinary"))) {
            throw new AssertionError("same-size symbols lost their identities: " + storage);
        }
        for (VariableStorage value : storage.values()) {
            if (value.size() != 72) throw new AssertionError("wrong logical size");
        }
        AddressSpace stack = currentProgram.getAddressFactory().getStackSpace();
        VariableStorage originalFirst = factory.getJoinStorage(new Varnode[] {
            new Varnode(stack.getAddress(0), 56), new Varnode(stack.getAddress(-16), 16) });
        VariableStorage originalSecond = factory.getJoinStorage(new Varnode[] {
            new Varnode(stack.getAddress(0), 60), new Varnode(stack.getAddress(-12), 12) });
        if (!originalFirst.equals(originalSecond)) {
            throw new AssertionError("the unadapted joins did not reproduce the collision");
        }
        println("WRAPPED_STACK_STORAGE_PASS " + storage);
    }
}
