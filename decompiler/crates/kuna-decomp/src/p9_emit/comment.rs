//! Port of `decompiler/cpp/comment.{cc,hh}` — the high-level-language comment
//! database and the comment sorter.
//!
//! Three pieces are transcribed here:
//!
//!   - [`Comment`]: a single comment, attached to a function address and an
//!     instruction address, with a type-flag word and a sub-sort `uniq` index.
//!   - [`CommentDatabaseInternal`] (behind the [`CommentDatabase`] trait): the
//!     in-memory, sorted container.  The C++ `set<Comment*,CommentOrder>` keyed
//!     on `(funcaddr, addr, uniq)` becomes a `BTreeMap<CommentKey, Comment>`
//!     with the same three-level comparator; the C++ `lower_bound` probes
//!     become half-open `range(..)` queries (a `lower_bound(beg) .. lower_bound(end)`
//!     pair is exactly a `beg..end` range over a sorted map).
//!   - [`CommentSorter`]: places comments into basic blocks / the header for
//!     emission, keyed on the `Subsort` `(index, order, pos)` tuple, and walks
//!     them with the `start`/`stop`/`opstop` cursor trio.
//!
//! ### Seams
//!
//! `CommentSorter::findPosition` / `setupFunctionList` read the live
//! [`Funcdata`] op-tree (`beginOp`/`endOpAll`/`beginOpAll`) and basic-block
//! geometry (`BlockBasic::contains`, `getIndex`, `op->getSeqNum().getOrder()`).
//! That live-IR query surface is captured by the [`SorterFuncdata`] trait so the
//! sorter can be exercised and verified without depending on the full
//! `Funcdata` mutation API; the W4/W5 wiring point implements [`SorterFuncdata`]
//! for the real `Funcdata`.  // STUB(W5)

use std::collections::BTreeMap;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::{
    Decoder, ElementId, Encoder, IdRegistry, ATTRIB_CONTENT, ATTRIB_TYPE,
};
use kuna_base::types::{int4, uint4};

// ---------------------------------------------------------------------------
// Marshaling ids (verbatim from comment.cc:21-23)
// ---------------------------------------------------------------------------

/// Marshaling element `<comment>` (`comment.cc:21`).
pub const ELEM_COMMENT: ElementId = ElementId::new("comment", 86);
/// Marshaling element `<commentdb>` (`comment.cc:22`).
pub const ELEM_COMMENTDB: ElementId = ElementId::new("commentdb", 87);
/// Marshaling element `<text>` (`comment.cc:23`).
pub const ELEM_TEXT: ElementId = ElementId::new("text", 88);

/// Register every comment marshaling id with an [`IdRegistry`] so a name-driven
/// `XmlDecode` resolves `<comment>`/`<commentdb>`/`<text>` (the C++ global
/// static-init side effect; the Rust registry is explicit).
pub fn register_ids(reg: &mut IdRegistry) {
    for e in [&ELEM_COMMENT, &ELEM_COMMENTDB, &ELEM_TEXT] {
        reg.register_element(e);
    }
}

// ---------------------------------------------------------------------------
// Comment (comment.hh:43-74, comment.cc:30-117)
// ---------------------------------------------------------------------------

/// Possible properties associated with a comment (C++ `Comment::comment_type`).
pub mod comment_type {
    use kuna_base::types::uint4;
    /// The first user defined property.
    pub const USER1: uint4 = 1;
    /// The second user defined property.
    pub const USER2: uint4 = 2;
    /// The third user defined property.
    pub const USER3: uint4 = 4;
    /// The comment should be displayed in the function header.
    pub const HEADER: uint4 = 8;
    /// The comment is auto-generated to alert the user.
    pub const WARNING: uint4 = 16;
    /// The comment is auto-generated and should be in the header.
    pub const WARNINGHEADER: uint4 = 32;
}

/// \brief A comment attached to a specific function and code address
///
/// Contains the actual character data of the comment.  It is fundamentally
/// attached to a specific function and to the address of an instruction (within
/// the function's body).  Comments can be categorized as a \e header (or not)
/// depending on whether it should be displayed as part of the general
/// description of the function or not.
///
/// The C++ `text` is a `std::string`; the port keeps it as a `Vec<u8>` so that
/// equality (the `addCommentNoDuplicate` text match) is byte-for-byte with the
/// C++.
#[derive(Debug, Clone)]
pub struct Comment {
    /// The properties associated with the comment.
    type_: uint4,
    /// Sub-identifier for uniqueness.
    uniq: int4,
    /// Address of the function containing the comment.
    funcaddr: Address,
    /// Address associated with the comment.
    addr: Address,
    /// The body of the comment.
    text: Vec<u8>,
    /// \b true if this comment has already been emitted.
    ///
    /// The C++ field is `mutable bool` (set through a `const Comment*`); the
    /// emitted flag is interior, sorter-driven state and is held by value here.
    emitted: bool,
}

impl Comment {
    /// Constructor (C++ `Comment(uint4 tp,const Address &fad,const Address &ad,int4 uq,const string &txt)`).
    pub fn new(tp: uint4, fad: Address, ad: Address, uq: int4, txt: &[u8]) -> Comment {
        Comment { type_: tp, uniq: uq, funcaddr: fad, addr: ad, text: txt.to_vec(), emitted: false }
    }

    /// Constructor for use with decode (C++ `Comment(void)`): all fields are
    /// overwritten by [`Comment::decode`].
    pub fn empty() -> Comment {
        Comment {
            type_: 0,
            uniq: 0,
            funcaddr: Address::default(),
            addr: Address::default(),
            text: Vec::new(),
            emitted: false,
        }
    }

    /// Mark that \b this comment has been emitted (C++ `setEmitted`, a `const`
    /// method mutating the `mutable emitted` field).
    pub fn set_emitted(&mut self, val: bool) {
        self.emitted = val;
    }

    /// Return \b true if \b this comment is already emitted (C++ `isEmitted`).
    pub fn is_emitted(&self) -> bool {
        self.emitted
    }

    /// Get the properties associated with the comment (C++ `getType`).
    pub fn get_type(&self) -> uint4 {
        self.type_
    }

    /// Get the address of the function containing the comment (C++ `getFuncAddr`).
    pub fn get_func_addr(&self) -> &Address {
        &self.funcaddr
    }

    /// Get the address to which the instruction is attached (C++ `getAddr`).
    pub fn get_addr(&self) -> &Address {
        &self.addr
    }

    /// Get the sub-sorting index (C++ `getUniq`).
    pub fn get_uniq(&self) -> int4 {
        self.uniq
    }

    /// Get the body of the comment (C++ `getText`).
    pub fn get_text(&self) -> &[u8] {
        &self.text
    }

    /// Encode the comment to a stream (C++ `Comment::encode`).
    ///
    /// The single comment is encoded as a `<comment>` element.  The two address
    /// children are written exactly as the C++ does (open `<addr>`, write the
    /// space's attributes, close) — `Address::encode` is the same sequence.
    pub fn encode(&self, encoder: &mut dyn Encoder) -> KunaResult<()> {
        let tpname = Comment::decode_comment_type(self.type_)?;
        encoder.open_element(&ELEM_COMMENT);
        encoder.write_string(&ATTRIB_TYPE, tpname.as_bytes());
        self.funcaddr.encode(encoder)?;
        self.addr.encode(encoder)?;
        encoder.open_element(&ELEM_TEXT);
        encoder.write_string(&ATTRIB_CONTENT, &self.text);
        encoder.close_element(&ELEM_TEXT);
        encoder.close_element(&ELEM_COMMENT);
        Ok(())
    }

    /// Decode a `<comment>` element from the given stream decoder
    /// (C++ `Comment::decode`).
    pub fn decode(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        self.emitted = false;
        self.type_ = 0;
        let elem_id = decoder.open_element_id(&ELEM_COMMENT)?;
        let tp = decoder.read_string_id(&ATTRIB_TYPE)?;
        self.type_ = Comment::encode_comment_type(&tp)?;
        self.funcaddr = Address::decode(decoder)?;
        self.addr = Address::decode(decoder)?;
        let sub_id = decoder.peek_element()?;
        if sub_id != 0 {
            decoder.open_element()?;
            self.text = decoder.read_string_id(&ATTRIB_CONTENT)?;
            decoder.close_element(sub_id)?;
        }
        decoder.close_element(elem_id)?;
        Ok(())
    }

    /// Convert a name string to a single comment property
    /// (C++ `Comment::encodeCommentType`).
    pub fn encode_comment_type(name: &[u8]) -> KunaResult<uint4> {
        if name == b"user1" {
            return Ok(comment_type::USER1);
        }
        if name == b"user2" {
            return Ok(comment_type::USER2);
        }
        if name == b"user3" {
            return Ok(comment_type::USER3);
        }
        if name == b"header" {
            return Ok(comment_type::HEADER);
        }
        if name == b"warning" {
            return Ok(comment_type::WARNING);
        }
        if name == b"warningheader" {
            return Ok(comment_type::WARNINGHEADER);
        }
        Err(KunaError::lowlevel(format!(
            "Unknown comment type: {}",
            String::from_utf8_lossy(name)
        )))
    }

    /// Convert a single comment property to its string representation
    /// (C++ `Comment::decodeCommentType`).
    pub fn decode_comment_type(val: uint4) -> KunaResult<&'static str> {
        match val {
            comment_type::USER1 => Ok("user1"),
            comment_type::USER2 => Ok("user2"),
            comment_type::USER3 => Ok("user3"),
            comment_type::HEADER => Ok("header"),
            comment_type::WARNING => Ok("warning"),
            comment_type::WARNINGHEADER => Ok("warningheader"),
            _ => Err(KunaError::lowlevel("Unknown comment type")),
        }
    }
}

// ---------------------------------------------------------------------------
// CommentOrder / CommentSet key (comment.hh:80-84, comment.cc:122-132)
// ---------------------------------------------------------------------------

/// Sorting key for the comment container (C++ `CommentOrder` over `Comment*`).
///
/// Comments are ordered first by function, then address, then the sub-sort
/// index — exactly the three-level `CommentOrder::operator()` (`a->getFuncAddr()`
/// `!=`/`<`, then `getAddr()`, then `getUniq()`, then `false`).  A `#[derive]`d
/// lexicographic `Ord` over `(Address, Address, int4)` is that same comparator;
/// the trailing "return false" is the BTreeMap's "keys equal" case.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CommentKey {
    /// Address of the function containing the comment.
    pub funcaddr: Address,
    /// Address associated with the comment.
    pub addr: Address,
    /// Sub-sort uniqueness index.
    pub uniq: int4,
}

impl CommentKey {
    fn of(comm: &Comment) -> CommentKey {
        CommentKey { funcaddr: comm.funcaddr.clone(), addr: comm.addr.clone(), uniq: comm.uniq }
    }
}

// ---------------------------------------------------------------------------
// CommentDatabase (comment.hh:92-176, comment.cc:134-263)
// ---------------------------------------------------------------------------

/// \brief An interface to a container of comments (C++ `CommentDatabase`).
///
/// Comments can be added (and removed) from a database, keying on the function
/// and address the Comment is attached to.  The interface can generate a \e
/// begin and \e end iterator covering all Comment objects for a single function;
/// the port exposes that range as [`CommentDatabase::comments_for`].
pub trait CommentDatabase {
    /// Clear all comments from this container (C++ `clear`).
    fn clear(&mut self);

    /// Clear all comments belonging to function \b fad that match (one of) the
    /// indicated property bits \b tp (C++ `clearType`).
    fn clear_type(&mut self, fad: &Address, tp: uint4);

    /// Add a new comment to the container (C++ `addComment`).
    fn add_comment(&mut self, tp: uint4, fad: &Address, ad: &Address, txt: &[u8]);

    /// Add a new comment, making sure there is no duplicate (C++
    /// `addCommentNoDuplicate`).  Returns `true` if a new Comment was created,
    /// `false` if there was a duplicate (same address, same body).
    fn add_comment_no_duplicate(
        &mut self,
        tp: uint4,
        fad: &Address,
        ad: &Address,
        txt: &[u8],
    ) -> bool;

    /// Remove the comment with the given key from the container (C++
    /// `deleteComment(Comment*)`; the port keys on the comment's [`CommentKey`]
    /// since there are no raw pointers).
    fn delete_comment(&mut self, key: &CommentKey);

    /// Iterate the comments for a single function, in container order (C++
    /// `beginComment(fad)`..`endComment(fad)`).
    fn comments_for<'a>(&'a self, fad: &Address) -> Box<dyn Iterator<Item = &'a Comment> + 'a>;

    /// Encode all comments in the container to a stream (C++ `encode`).
    fn encode(&self, encoder: &mut dyn Encoder) -> KunaResult<()>;

    /// Decode all comments from a `<commentdb>` element (C++ `decode`).
    fn decode(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()>;
}

/// \brief An in-memory implementation of the [`CommentDatabase`] API
/// (C++ `CommentDatabaseInternal`).
///
/// All Comment objects are held in memory in a sorted container.  The C++
/// `set<Comment*,CommentOrder>` becomes a `BTreeMap<CommentKey, Comment>`: the
/// key is the comparator's `(funcaddr, addr, uniq)` triple, so iteration order
/// and the `lower_bound`/`upper_bound`/`range` probes line up one-to-one.
#[derive(Debug, Default)]
pub struct CommentDatabaseInternal {
    /// The sorted set of Comment objects.
    commentset: BTreeMap<CommentKey, Comment>,
}

impl CommentDatabaseInternal {
    /// Constructor (C++ `CommentDatabaseInternal(void)`).
    pub fn new() -> CommentDatabaseInternal {
        CommentDatabaseInternal { commentset: BTreeMap::new() }
    }

    /// The half-open key range covering every comment of function \b fad.
    ///
    /// C++ `beginComment`/`endComment` build probe Comments at
    /// `(fad, m_minimal, 0)` and `(fad, m_maximal, 65535)` and `lower_bound`
    /// them; the half-open `[beg, end)` over a sorted map is the identical span.
    fn func_key_range(fad: &Address) -> (CommentKey, CommentKey) {
        let beg = CommentKey {
            funcaddr: fad.clone(),
            addr: Address::new_extreme(kuna_base::address::mach_extreme::m_minimal),
            uniq: 0,
        };
        let end = CommentKey {
            funcaddr: fad.clone(),
            addr: Address::new_extreme(kuna_base::address::mach_extreme::m_maximal),
            uniq: 65535,
        };
        (beg, end)
    }
}

impl CommentDatabase for CommentDatabaseInternal {
    fn clear(&mut self) {
        self.commentset.clear();
    }

    fn clear_type(&mut self, fad: &Address, tp: uint4) {
        // C++ walks [lower_bound(testcommbeg), lower_bound(testcommend)) and
        // erases the entries whose type intersects `tp`.  Collect-then-remove
        // avoids mutating the map while iterating.
        let (beg, end) = CommentDatabaseInternal::func_key_range(fad);
        let to_remove: Vec<CommentKey> = self
            .commentset
            .range(beg..end)
            .filter(|(_, c)| (c.get_type() & tp) != 0)
            .map(|(k, _)| k.clone())
            .collect();
        for k in to_remove {
            self.commentset.remove(&k);
        }
    }

    fn add_comment(&mut self, tp: uint4, fad: &Address, ad: &Address, txt: &[u8]) {
        // C++: build the probe Comment at uniq 65535, find lower_bound (first
        // element >= probe), step back one to the "last element less than", and
        // — if that predecessor shares (addr, funcaddr) — set uniq to its
        // uniq+1.  Reproduced via the BTreeMap range cursor.
        let mut newcom = Comment::new(tp, fad.clone(), ad.clone(), 65535, txt);
        let probe = CommentKey::of(&newcom);
        // First element less than the probe (predecessor of lower_bound).
        let pred = self.commentset.range(..probe).next_back();
        newcom.uniq = 0;
        if let Some((_, prev)) = pred {
            if prev.get_addr() == ad && prev.get_func_addr() == fad {
                newcom.uniq = prev.get_uniq() + 1;
            }
        }
        self.commentset.insert(CommentKey::of(&newcom), newcom);
    }

    fn add_comment_no_duplicate(
        &mut self,
        tp: uint4,
        fad: &Address,
        ad: &Address,
        txt: &[u8],
    ) -> bool {
        // C++: probe at uniq 65535, then walk backwards from lower_bound while
        // each predecessor still matches (addr, funcaddr): if any has identical
        // text, it's a duplicate (abort); otherwise bump uniq to the first
        // (highest) predecessor's uniq+1.  The walk stops at the first
        // non-matching predecessor.
        let mut newcom = Comment::new(tp, fad.clone(), ad.clone(), 65535, txt);
        let probe = CommentKey::of(&newcom);
        newcom.uniq = 0; // set the uniq AFTER the search probe is built
        for (_, prev) in self.commentset.range(..probe).rev() {
            if prev.get_addr() == ad && prev.get_func_addr() == fad {
                if prev.get_text() == txt {
                    // Matching text, don't store it.
                    return false;
                }
                if newcom.uniq == 0 {
                    newcom.uniq = prev.get_uniq() + 1;
                }
            } else {
                break;
            }
        }
        self.commentset.insert(CommentKey::of(&newcom), newcom);
        true
    }

    fn delete_comment(&mut self, key: &CommentKey) {
        self.commentset.remove(key);
    }

    fn comments_for<'a>(&'a self, fad: &Address) -> Box<dyn Iterator<Item = &'a Comment> + 'a> {
        let (beg, end) = CommentDatabaseInternal::func_key_range(fad);
        Box::new(self.commentset.range(beg..end).map(|(_, c)| c))
    }

    fn encode(&self, encoder: &mut dyn Encoder) -> KunaResult<()> {
        encoder.open_element(&ELEM_COMMENTDB);
        for comm in self.commentset.values() {
            comm.encode(encoder)?;
        }
        encoder.close_element(&ELEM_COMMENTDB);
        Ok(())
    }

    fn decode(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        let elem_id = decoder.open_element_id(&ELEM_COMMENTDB)?;
        while decoder.peek_element()? != 0 {
            let mut com = Comment::empty();
            com.decode(decoder)?;
            self.add_comment(com.get_type(), com.get_func_addr(), com.get_addr(), com.get_text());
        }
        decoder.close_element(elem_id)?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// CommentSorter (comment.hh:178-252, comment.cc:265-404)
// ---------------------------------------------------------------------------

/// Header-comment sub-categories (C++ anonymous enum in `CommentSorter`).
pub mod header_kind {
    use kuna_base::types::uint4;
    /// Basic header comments.
    pub const HEADER_BASIC: uint4 = 0;
    /// Comment that can't be placed in code flow.
    pub const HEADER_UNPLACED: uint4 = 1;
}

/// \brief The sorting key for placing a Comment within a specific basic block
/// (C++ `CommentSorter::Subsort`).
///
/// `index` is the basic block index (or -1 for a function header), `order` is
/// the position within the block, `pos` is a final count guaranteeing a unique
/// sort.  The `#[derive]`d lexicographic `Ord` over `(int4, uint4, uint4)` is
/// exactly `Subsort::operator<` (index, then order, then pos).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Subsort {
    /// Either the basic block index or -1 for a function header.
    pub index: int4,
    /// The order index within the basic block.
    pub order: uint4,
    /// A final count to guarantee a unique sorting.
    pub pos: uint4,
}

impl Subsort {
    /// A fresh key (the C++ struct is left uninitialized; callers always set
    /// the fields they consult before use).
    fn new() -> Subsort {
        Subsort { index: 0, order: 0, pos: 0 }
    }

    /// Initialize a key for a header comment (C++ `setHeader`).
    fn set_header(&mut self, header_type: uint4) {
        self.index = -1; // -1 indicates a header comment
        self.order = header_type;
    }

    /// Initialize a key for a basic block position (C++ `setBlock`).
    fn set_block(&mut self, i: int4, ord: uint4) {
        self.index = i;
        self.order = ord;
    }
}

/// \brief The live-IR query surface the [`CommentSorter`] needs from a function.
///
/// STUB(W5): `CommentSorter::findPosition`/`setupFunctionList` read the op-tree
/// and basic-block geometry of a live [`Funcdata`].  The exact C++ calls are
/// captured here so the sorter can be transcribed and verified independently of
/// the full `Funcdata` API; the integration point implements this trait for the
/// real `Funcdata`.
///
/// All op references are the abstract index `usize` returned by the cursor
/// methods; the trait never exposes a concrete op type.
///
/// The C++ uses a single `PcodeOpTree::const_iterator opiter` and tests it
/// against the sentinels `endOpAll()` / `beginOpAll()`.  That iterator's three
/// states are modeled by [`OpProbe`]: `at_end` is `opiter == endOpAll()`, and a
/// real [`OpCursor`] otherwise.  [`SorterFuncdata::prev_op`] performs the C++
/// `--opiter` (guarded by `opiter != beginOpAll()`), which works on both an
/// at-end probe (giving the last op of a non-empty tree) and a real cursor
/// (giving the op before it).
pub trait SorterFuncdata {
    /// The address of the function (C++ `Funcdata::getAddress`).
    fn get_address(&self) -> &Address;

    /// `true` if the op-tree is empty (C++ `beginOpAll() == endOpAll()`).
    fn op_tree_empty(&self) -> bool;

    /// `beginOp(addr)` as a probe: the op at the lowest address `>= addr`, or an
    /// at-end probe if no op is at or after `addr` (C++ `opiter` vs `endOpAll()`).
    fn first_op_at_or_after(&self, addr: &Address) -> OpProbe;

    /// The C++ `--opiter`, guarded by `opiter != beginOpAll()`: the op
    /// immediately before the probe, or `None` if the probe is already at
    /// `beginOpAll()`.  When the probe is at-end on a non-empty tree this yields
    /// the last op; on an empty tree it yields `None`.
    fn prev_op(&self, probe: &OpProbe) -> Option<OpCursor>;

    /// The block index, the op's seqnum order, and whether the op's basic block
    /// `contains` the comment address, for the op at the given cursor (C++
    /// `op->getParent()->getIndex()`, `op->getSeqNum().getOrder()`,
    /// `block->contains(addr)`).  A dead op (no parent) is the C++
    /// `LowlevelError("Dead op reaching CommentSorter")` and surfaces as `Err`.
    fn op_block_info(&self, cursor: &OpCursor, addr: &Address) -> KunaResult<OpBlockInfo>;

    /// The address of the op at the given cursor (C++ `op->getAddr()`).
    fn op_addr(&self, cursor: &OpCursor) -> Address;
}

/// An opaque cursor into a [`SorterFuncdata`]'s op-tree (the C++
/// `PcodeOpTree::const_iterator`).  // STUB(W5)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpCursor(pub usize);

/// The result of `beginOp(addr)`: either a real [`OpCursor`] or the at-end
/// sentinel (`opiter == endOpAll()`).  Carries enough to run the C++
/// `--opiter` in [`SorterFuncdata::prev_op`].  // STUB(W5)
#[derive(Debug, Clone, Copy)]
pub enum OpProbe {
    /// `opiter` points at a real op (the op at or after the comment address).
    At(OpCursor),
    /// `opiter == endOpAll()` (nothing at or after the comment address).
    AtEnd,
}

/// The block placement facts the sorter reads off a single op (C++
/// `op->getParent()->getIndex()`, `op->getSeqNum().getOrder()`,
/// `block->contains(addr)`).
#[derive(Debug, Clone, Copy)]
pub struct OpBlockInfo {
    /// Index of the op's basic block (C++ `block->getIndex()`).
    pub block_index: int4,
    /// The op's seqnum order within its block (C++ `op->getSeqNum().getOrder()`).
    pub order: uint4,
    /// Whether the op's block contains the comment address (C++ `block->contains(addr)`).
    pub block_contains_comment: bool,
}

/// \brief A class for sorting comments into and within basic blocks
/// (C++ `CommentSorter`).
///
/// Sorts comments into the basic block that contains them, so that — as
/// statements are emitted — comments are picked up in the correct order even
/// after the original p-code op at the comment's address has been removed or
/// migrated.  Also acts as the walk state for a specific basic block or the
/// header.
///
/// The C++ holds three `map::const_iterator`s (`start`/`stop`/`opstop`) into
/// `commmap`.  Borrow-checking forbids storing live `BTreeMap` cursors next to
/// the map, so the port keeps the three positions as the [`Subsort`] *keys* that
/// bound each walk and resolves them against `commmap` on demand — the same
/// positions, expressed by value.
pub struct CommentSorter {
    /// Comments for the current function, sorted by block.
    commmap: BTreeMap<Subsort, Comment>,
    /// Lower bound (inclusive) of the current walk (C++ `start`).
    start: Option<Subsort>,
    /// Upper bound (exclusive) of the current block set (C++ `stop`).
    stop: Option<Subsort>,
    /// Statement landmark / walk terminator within the current set (C++ `opstop`).
    opstop: Option<Subsort>,
    /// Key of the comment most recently returned by `get_next` (so the printer's
    /// `emitLineComment` can flip its `emitted` flag).  Not part of C++ — the C++
    /// printer holds the `const Comment*` directly; the port re-resolves by key.
    last_returned: Option<Subsort>,
    /// True if unplaced comments should be displayed (in the header).
    display_unplaced_comments: bool,
}

impl Default for CommentSorter {
    fn default() -> CommentSorter {
        CommentSorter::new()
    }
}

impl CommentSorter {
    /// Constructor (C++ `CommentSorter(void)`): `displayUnplacedComments = false`.
    pub fn new() -> CommentSorter {
        CommentSorter {
            commmap: BTreeMap::new(),
            start: None,
            stop: None,
            opstop: None,
            last_returned: None,
            display_unplaced_comments: false,
        }
    }

    /// Figure out the position of the given Comment and initialize its key
    /// (C++ `CommentSorter::findPosition`).  Returns `Ok(true)` and fills
    /// `subsort` if the Comment could be positioned at all, `Ok(false)` if its
    /// basic block has been excised (and unplaced comments are not displayed).
    fn find_position(
        &self,
        subsort: &mut Subsort,
        comm: &Comment,
        fd: &dyn SorterFuncdata,
    ) -> KunaResult<bool> {
        if comm.get_type() == 0 {
            return Ok(false);
        }
        let fad = fd.get_address().clone();
        if (comm.get_type() & (comment_type::HEADER | comment_type::WARNINGHEADER)) != 0
            && *comm.get_addr() == fad
        {
            // Header comment at the address of the start of the function.
            subsort.set_header(header_kind::HEADER_BASIC);
            return Ok(true);
        }

        // Try to find the block containing the comment.  Find the op at the
        // lowest address greater than or equal to the comment's address
        // (C++ `opiter = fd->beginOp(comm->getAddr())`).
        let opiter = fd.first_op_at_or_after(comm.get_addr());
        let mut backup_op: Option<OpCursor> = None;
        if let OpProbe::At(cursor) = opiter {
            // If there is an op at or after the comment.
            let info = fd.op_block_info(&cursor, comm.get_addr())?;
            if info.block_contains_comment {
                // The op's block contains the address: associate with this op.
                subsort.set_block(info.block_index, info.order);
                return Ok(true);
            }
            if *comm.get_addr() == fd.op_addr(&cursor) {
                backup_op = Some(cursor);
            }
        }
        // If there is a previous op (C++ `if (opiter != beginOpAll()) { --opiter; ...}`).
        // `prev_op` performs the guarded `--opiter` on the probe, so the at-end
        // probe over a non-empty tree decrements to the last op, exactly as in
        // the C++.
        let prev = fd.prev_op(&opiter);
        if let Some(cursor) = prev {
            let info = fd.op_block_info(&cursor, comm.get_addr())?;
            if info.block_contains_comment {
                // Treat the comment as being in this block at the very end.
                subsort.set_block(info.block_index, 0xffffffff);
                return Ok(true);
            }
        }
        if let Some(cursor) = backup_op {
            // The op may have migrated from its original basic block.  Since the
            // address matches exactly, hang the comment on it.
            let info = fd.op_block_info(&cursor, comm.get_addr())?;
            subsort.set_block(info.block_index, info.order);
            return Ok(true);
        }
        if fd.op_tree_empty() {
            // If there are no ops at all, put the comment at the beginning of
            // the first block.
            subsort.set_block(0, 0);
            return Ok(true);
        }
        if self.display_unplaced_comments {
            subsort.set_header(header_kind::HEADER_UNPLACED);
            return Ok(true);
        }
        Ok(false) // Basic block containing comment has been excised.
    }

    /// Collect and sort comments specific to the given function
    /// (C++ `CommentSorter::setupFunctionList`).
    ///
    /// Only keeps comments matching one of the property bits in `tp`.
    pub fn setup_function_list(
        &mut self,
        tp: uint4,
        fd: &dyn SorterFuncdata,
        db: &dyn CommentDatabase,
        display_unplaced: bool,
    ) -> KunaResult<()> {
        self.commmap.clear();
        self.display_unplaced_comments = display_unplaced;
        if tp == 0 {
            return Ok(());
        }
        let fad = fd.get_address().clone();
        let mut subsort = Subsort::new();
        subsort.pos = 0;

        for comm in db.comments_for(&fad) {
            if self.find_position(&mut subsort, comm, fd)? {
                let mut placed = comm.clone();
                placed.set_emitted(false);
                self.commmap.insert(subsort, placed);
                subsort.pos += 1; // Advance the uniqueness counter
            }
        }
        Ok(())
    }

    /// Establish a p-code landmark within the current set of comments
    /// (C++ `CommentSorter::setupOpList`).
    ///
    /// Generally called with the root p-code op of a statement being emitted;
    /// it sets the key at which to stop emitting comments for the statement.
    /// A `None` op picks up any remaining comments in the basic block.
    pub fn setup_op_list(&mut self, op: Option<OpListLandmark>) {
        match op {
            None => {
                // pick up any remaining comments in this basic block
                self.opstop = self.stop;
            }
            Some(landmark) => {
                let subsort = Subsort { index: landmark.index, order: landmark.order, pos: 0xffffffff };
                // C++ `opstop = commmap.upper_bound(subsort)`: first key strictly
                // greater than `subsort`.
                self.opstop = self.upper_bound(&subsort);
            }
        }
    }

    /// Find iterators that bound everything in the basic block
    /// (C++ `CommentSorter::setupBlockList`).
    pub fn setup_block_list(&mut self, block_index: int4) {
        let mut subsort = Subsort { index: block_index, order: 0, pos: 0 };
        self.start = self.lower_bound(&subsort);
        subsort.order = 0xffffffff;
        subsort.pos = 0xffffffff;
        self.stop = self.upper_bound(&subsort);
    }

    /// Header comments are grouped together; set up the walk iterators
    /// (C++ `CommentSorter::setupHeader`).
    pub fn setup_header(&mut self, header_type: uint4) {
        let mut subsort = Subsort { index: -1, order: header_type, pos: 0 };
        self.start = self.lower_bound(&subsort);
        subsort.pos = 0xffffffff;
        self.opstop = self.upper_bound(&subsort);
    }

    /// Return `true` if there are more comments to emit in the current set
    /// (C++ `hasNext`: `start != opstop`).
    pub fn has_next(&self) -> bool {
        self.start != self.opstop
    }

    /// Advance to and return the next comment (C++ `getNext`:
    /// `res = (*start).second; ++start; return res`).
    ///
    /// Returns a reference into `commmap`; `start` advances to the next key.
    /// Records the returned key so [`mark_last_emitted`](CommentSorter::mark_last_emitted)
    /// can flip its `emitted` flag (the C++ `emitLineComment` `setEmitted(true)`).
    pub fn get_next(&mut self) -> &Comment {
        let key = self.start.expect("CommentSorter::get_next past end (C++ UB on ++start)");
        // ++start: the next key strictly greater than the current `start`.
        self.start = self.successor(&key);
        self.last_returned = Some(key);
        // Resolve the comment at the original `start` key.
        self.commmap.get(&key).expect("CommentSorter::get_next: dangling start key")
    }

    /// Mark the comment most recently returned by [`get_next`](CommentSorter::get_next)
    /// as emitted (C++ `Comment::setEmitted(true)`, called from
    /// `emitLineComment`).  A later walk over the same window then skips it.
    pub fn mark_last_emitted(&mut self) {
        if let Some(key) = self.last_returned {
            if let Some(c) = self.commmap.get_mut(&key) {
                c.set_emitted(true);
            }
        }
    }

    // --- BTreeMap cursor helpers (the C++ map::lower_bound/upper_bound) ------

    /// First key `>= subsort` (C++ `map::lower_bound`), or `None` past the end.
    fn lower_bound(&self, subsort: &Subsort) -> Option<Subsort> {
        self.commmap.range(*subsort..).next().map(|(k, _)| *k)
    }

    /// First key `> subsort` (C++ `map::upper_bound`), or `None` past the end.
    fn upper_bound(&self, subsort: &Subsort) -> Option<Subsort> {
        use std::ops::Bound::{Excluded, Unbounded};
        self.commmap.range((Excluded(*subsort), Unbounded)).next().map(|(k, _)| *k)
    }

    /// Key strictly after `key` (the C++ `++iter`).
    fn successor(&self, key: &Subsort) -> Option<Subsort> {
        use std::ops::Bound::{Excluded, Unbounded};
        self.commmap.range((Excluded(*key), Unbounded)).next().map(|(k, _)| *k)
    }
}

/// The p-code landmark passed to [`CommentSorter::setup_op_list`] (C++
/// `op->getParent()->getIndex()` / `op->getSeqNum().getOrder()`).  // STUB(W5)
#[derive(Debug, Clone, Copy)]
pub struct OpListLandmark {
    /// The op's basic-block index.
    pub index: int4,
    /// The op's seqnum order within the block.
    pub order: uint4,
}

#[cfg(test)]
mod tests;
