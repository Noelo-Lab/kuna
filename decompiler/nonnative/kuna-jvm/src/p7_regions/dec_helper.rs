//! Port of `org/jetbrains/java/decompiler/modules/decompiler/DecHelper.java`.

use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::rt::JHashSet;
use crate::stat_edge::StatEdge;
use crate::statement::{EdgeDirection, LastBasicType, Statement, StatementType};

pub struct DecHelper;

impl DecHelper {
    pub fn check_statement_exceptions(lst: &[Statement]) -> bool {
        let all: JHashSet<Statement> = JHashSet::from_collection(lst.iter().copied());

        let mut handlers: JHashSet<Statement> = JHashSet::new();
        let mut intersection: Option<JHashSet<Statement>> = None;

        for stat in lst.iter().copied() {
            let mut set_new =
                stat.get_neighbours_set(StatEdge::TYPE_EXCEPTION, EdgeDirection::Forward);

            match intersection.as_mut() {
                None => {
                    intersection = Some(set_new);
                }
                Some(intersection) => {
                    let mut interclone = intersection.clone();
                    interclone.remove_all(&set_new);

                    intersection.retain_all(&set_new);

                    set_new.remove_all(intersection);

                    handlers.add_all(interclone.to_vec());
                    handlers.add_all(set_new.to_vec());
                }
            }
        }

        for stat in handlers.to_vec() {
            if !all.contains(&stat)
                || !stat
                    .get_neighbours(StatEdge::TYPE_EXCEPTION, EdgeDirection::Backward)
                    .iter()
                    .all(|s| all.contains(s))
            {
                return false;
            }
        }

        for stat in lst.iter().copied().skip(1) {
            if !stat
                .get_predecessor_edges(StatEdge::TYPE_EXCEPTION)
                .is_empty()
                && !handlers.contains(&stat)
            {
                return false;
            }
        }

        true
    }

    /// Java: isChoiceStatement(Statement, List). On success `lst` holds the post
    /// statement (possibly null) at index 0 followed by the choice branches.
    pub fn is_choice_statement(head: Statement, lst: &mut Vec<Option<Statement>>) -> bool {
        let mut post: Option<Statement> = None;

        let mut set_dest = head.get_neighbours_set(StatEdge::TYPE_REGULAR, EdgeDirection::Forward);

        if set_dest.contains(&head) {
            return false;
        }

        loop {
            lst.clear();

            let mut repeat = false;

            if let Some(p) = post {
                set_dest.remove(&p);
            }

            for stat in set_dest.to_vec() {
                if stat.get_last_basic_type() != LastBasicType::General {
                    if post.is_none() {
                        post = Some(stat);
                        repeat = true;
                        break;
                    } else {
                        return false;
                    }
                }

                let mut set_pred =
                    stat.get_neighbours_set(StatEdge::TYPE_REGULAR, EdgeDirection::Backward);
                set_pred.remove(&head);
                if set_pred.contains(&stat) {
                    return false;
                }

                if !set_dest.contains_all(&set_pred) || set_pred.len() > 1 {
                    if post.is_none() {
                        post = Some(stat);
                        repeat = true;
                        break;
                    } else {
                        return false;
                    }
                } else if set_pred.len() == 1 {
                    let mut pred = set_pred.iter().next().copied().unwrap();
                    while lst.contains(&Some(pred)) {
                        let mut set_pred_temp = pred
                            .get_neighbours_set(StatEdge::TYPE_REGULAR, EdgeDirection::Backward);
                        set_pred_temp.remove(&head);

                        if !set_pred_temp.is_empty() {
                            pred = set_pred_temp.iter().next().copied().unwrap();
                            if pred == stat {
                                return false;
                            }
                        } else {
                            break;
                        }
                    }
                }

                let lst_edges = stat.get_successor_edges(Statement::STATEDGE_DIRECT_ALL);
                if lst_edges.len() > 1 {
                    let mut set_succ = stat
                        .get_neighbours_set(Statement::STATEDGE_DIRECT_ALL, EdgeDirection::Forward);
                    set_succ.retain_all(&set_dest);

                    if !set_succ.is_empty() {
                        return false;
                    } else if post.is_none() {
                        post = Some(stat);
                        repeat = true;
                        break;
                    } else {
                        return false;
                    }
                } else if lst_edges.len() == 1 {
                    let edge = lst_edges[0];
                    if edge.get_type() == StatEdge::TYPE_REGULAR {
                        let statd = edge.get_destination();
                        if head == statd {
                            return false;
                        }
                        if post != Some(statd) && !set_dest.contains(&statd) {
                            if post.is_some() {
                                return false;
                            } else {
                                let set = statd.get_neighbours_set(
                                    StatEdge::TYPE_REGULAR,
                                    EdgeDirection::Backward,
                                );
                                if set.len() > 1 {
                                    post = Some(statd);
                                    repeat = true;
                                    break;
                                } else {
                                    return false;
                                }
                            }
                        }
                    }
                }

                lst.push(Some(stat));
            }

            if !repeat {
                break;
            }
        }

        lst.push(Some(head));
        if let Some(i) = lst.iter().position(|s| *s == post) {
            lst.remove(i);
        }

        lst.insert(0, post);

        true
    }

    pub fn get_unique_pred_exceptions(head: Statement) -> JHashSet<Statement> {
        let mut set_handlers: JHashSet<Statement> = JHashSet::from_collection(
            head.get_neighbours(StatEdge::TYPE_EXCEPTION, EdgeDirection::Forward),
        );
        set_handlers.retain(|statement| {
            statement
                .get_predecessor_edges(StatEdge::TYPE_EXCEPTION)
                .len()
                <= 1
        });
        set_handlers
    }

    pub fn invalid_head_merge(head: Statement) -> bool {
        if DecompilerContext::get_option(IFernflowerPreferences::TRY_LOOP_FIX) {
            let ifhead = Self::find_if_head(head);

            if let Some(ifhead) = ifhead {
                if let Some(first) = ifhead.get_first() {
                    if head.get_continue_set().borrow().contains(&first) {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn find_if_head(head: Statement) -> Option<Statement> {
        let mut head = Some(head);
        while let Some(h) = head {
            let type_ = h.with(|d| d.type_);
            if type_ == StatementType::If {
                break;
            }
            if type_ != StatementType::Sequence {
                return None;
            }

            head = h.get_first();
        }

        head
    }

    /// Java: copyExprentList(List). Takes a slice so both `JList` exprent lists
    /// (`&list.to_vec()`) and plain vectors can be copied.
    pub fn copy_exprent_list(lst: &[Exprent]) -> Vec<Exprent> {
        let mut ret: Vec<Exprent> = Vec::new();
        for expr in lst.iter().copied() {
            ret.push(expr.copy());
        }
        ret
    }
}
