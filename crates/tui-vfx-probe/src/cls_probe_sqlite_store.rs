// <FILE>crates/tui-vfx-probe/src/cls_probe_sqlite_store.rs</FILE> - <DESC>In-memory SQLite index for probe playback data</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Background snapshot storage for trace-event SQLite queries</WCTX>
// <CLOG>MINOR: Extend probe_trace_events storage to persist before/after background color snapshots so background-only effects can be verified directly through SQL queries</CLOG>

use rusqlite::types::ValueRef;
use rusqlite::{Connection, params};
use serde_json::{Map, Value, json};

use crate::{ProbeDiffReport, ProbeReport, ProbeTimelineReport};

pub struct ProbeSqliteStore {
    conn: Connection,
}

impl ProbeSqliteStore {
    pub fn new_in_memory() -> Result<Self, rusqlite::Error> {
        let conn = Connection::open_in_memory()?;
        let store = Self { conn };
        store.create_schema()?;
        Ok(store)
    }

    pub fn ingest_report(&self, run_id: &str, report: &ProbeReport) -> Result<(), rusqlite::Error> {
        self.conn.execute(
            "insert into probe_runs(run_id, kind, source_input_kind) values (?1, ?2, ?3)",
            params![run_id, report.kind, report.source.input_kind],
        )?;
        self.insert_frame(run_id, 0, report)?;
        Ok(())
    }

    pub fn ingest_timeline(
        &self,
        run_id: &str,
        timeline: &ProbeTimelineReport,
    ) -> Result<(), rusqlite::Error> {
        self.conn.execute(
            "insert into probe_runs(run_id, kind, source_input_kind, phase, frame_count) values (?1, ?2, ?3, ?4, ?5)",
            params![run_id, timeline.kind, timeline.source.input_kind, format!("{:?}", timeline.phase), timeline.frame_count as i64],
        )?;
        for (frame_index, frame) in timeline.frames.iter().enumerate() {
            self.insert_frame(run_id, frame_index as i64, frame)?;
        }
        Ok(())
    }

    pub fn ingest_diff(&self, run_id: &str, diff: &ProbeDiffReport) -> Result<(), rusqlite::Error> {
        self.conn.execute(
            "insert into probe_runs(run_id, kind, source_input_kind, phase, from_t, to_t) values (?1, ?2, ?3, ?4, ?5, ?6)",
            params![run_id, diff.kind, diff.source.input_kind, format!("{:?}", diff.phase), diff.from_t, diff.to_t],
        )?;
        for cell in &diff.cells {
            self.conn.execute(
                "insert into probe_diff_cells(run_id, abs_x, abs_y, widget_x, widget_y, before_ch, after_ch, before_fg_r, before_fg_g, before_fg_b, before_fg_a, after_fg_r, after_fg_g, after_fg_b, after_fg_a, last_touch_stage, last_touch_effect, trace_len) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)",
                params![
                    run_id,
                    cell.abs.x as i64,
                    cell.abs.y as i64,
                    cell.widget_local.x as i64,
                    cell.widget_local.y as i64,
                    cell.before.ch.map(|value| value.to_string()),
                    cell.after.ch.map(|value| value.to_string()),
                    cell.before.fg.r as i64,
                    cell.before.fg.g as i64,
                    cell.before.fg.b as i64,
                    cell.before.fg.a as i64,
                    cell.after.fg.r as i64,
                    cell.after.fg.g as i64,
                    cell.after.fg.b as i64,
                    cell.after.fg.a as i64,
                    cell.last_touch.as_ref().map(|value| value.stage.clone()),
                    cell.last_touch.as_ref().and_then(|value| value.effect.clone()),
                    cell.trace.len() as i64,
                ],
            )?;
        }
        Ok(())
    }

    pub fn query_json(&self, sql: &str) -> Result<Vec<Value>, rusqlite::Error> {
        let mut stmt = self.conn.prepare(sql)?;
        let column_names = stmt
            .column_names()
            .iter()
            .map(|name| name.to_string())
            .collect::<Vec<_>>();
        let mut rows = stmt.query([])?;
        let mut out = Vec::new();
        while let Some(row) = rows.next()? {
            let mut object = Map::new();
            for (index, name) in column_names.iter().enumerate() {
                let value = match row.get_ref(index)? {
                    ValueRef::Null => Value::Null,
                    ValueRef::Integer(v) => json!(v),
                    ValueRef::Real(v) => json!(v),
                    ValueRef::Text(v) => json!(String::from_utf8_lossy(v).to_string()),
                    ValueRef::Blob(v) => json!(v.to_vec()),
                };
                object.insert(name.clone(), value);
            }
            out.push(Value::Object(object));
        }
        Ok(out)
    }

    fn create_schema(&self) -> Result<(), rusqlite::Error> {
        self.conn.execute_batch(
            "
            create table probe_runs (
                run_id text primary key,
                kind text not null,
                source_input_kind text not null,
                phase text,
                frame_count integer,
                from_t real,
                to_t real
            );
            create table probe_frames (
                run_id text not null,
                frame_index integer not null,
                requested_phase text not null,
                requested_t real not null,
                effective_phase text not null,
                effective_t real not null,
                frame_width integer not null,
                frame_height integer not null,
                widget_abs_x integer not null,
                widget_abs_y integer not null,
                widget_width integer not null,
                widget_height integer not null,
                total_cells integer not null,
                non_empty_cells integer not null,
                modified_cells integer not null,
                sampler text,
                mask_count integer not null,
                filter_count integer not null,
                shader_count integer not null,
                style_count integer not null,
                content_count integer not null
            );
            create table probe_cells (
                run_id text not null,
                frame_index integer not null,
                abs_x integer not null,
                abs_y integer not null,
                widget_x integer not null,
                widget_y integer not null,
                ch text not null,
                fg_r integer not null,
                fg_g integer not null,
                fg_b integer not null,
                fg_a integer not null,
                bg_r integer not null,
                bg_g integer not null,
                bg_b integer not null,
                bg_a integer not null,
                modifiers_json text not null,
                last_touch_stage text,
                last_touch_effect text
            );
            create table probe_trace_events (
                run_id text not null,
                frame_index integer not null,
                widget_x integer not null,
                widget_y integer not null,
                event_index integer not null,
                stage text not null,
                effect text,
                sampled_from_x integer,
                sampled_from_y integer,
                visible integer,
                before_fg_r integer,
                before_fg_g integer,
                before_fg_b integer,
                before_fg_a integer,
                before_bg_r integer,
                before_bg_g integer,
                before_bg_b integer,
                before_bg_a integer,
                after_fg_r integer,
                after_fg_g integer,
                after_fg_b integer,
                after_fg_a integer,
                after_bg_r integer,
                after_bg_g integer,
                after_bg_b integer,
                after_bg_a integer
            );
            create table probe_diff_cells (
                run_id text not null,
                abs_x integer not null,
                abs_y integer not null,
                widget_x integer not null,
                widget_y integer not null,
                before_ch text,
                after_ch text,
                before_fg_r integer not null,
                before_fg_g integer not null,
                before_fg_b integer not null,
                before_fg_a integer not null,
                after_fg_r integer not null,
                after_fg_g integer not null,
                after_fg_b integer not null,
                after_fg_a integer not null,
                last_touch_stage text,
                last_touch_effect text,
                trace_len integer not null
            );
            ",
        )?;
        Ok(())
    }

    fn insert_frame(
        &self,
        run_id: &str,
        frame_index: i64,
        report: &ProbeReport,
    ) -> Result<(), rusqlite::Error> {
        self.conn.execute(
            "insert into probe_frames(run_id, frame_index, requested_phase, requested_t, effective_phase, effective_t, frame_width, frame_height, widget_abs_x, widget_abs_y, widget_width, widget_height, total_cells, non_empty_cells, modified_cells, sampler, mask_count, filter_count, shader_count, style_count, content_count) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21)",
            params![
                run_id,
                frame_index,
                format!("{:?}", report.request.phase),
                report.request.sample_t,
                format!("{:?}", report.timing.effective_phase),
                report.timing.effective_t,
                report.frame.size.width as i64,
                report.frame.size.height as i64,
                report.widget.abs_origin.x as i64,
                report.widget.abs_origin.y as i64,
                report.widget.size.width as i64,
                report.widget.size.height as i64,
                report.summary.total_cells as i64,
                report.summary.non_empty_cells as i64,
                report.summary.modified_cells as i64,
                report.pipeline.sampler.clone(),
                report.pipeline.mask_count as i64,
                report.pipeline.filter_count as i64,
                report.pipeline.shader_count as i64,
                report.pipeline.style_count as i64,
                report.pipeline.content_count as i64,
            ],
        )?;
        for cell in &report.cells {
            self.conn.execute(
                "insert into probe_cells(run_id, frame_index, abs_x, abs_y, widget_x, widget_y, ch, fg_r, fg_g, fg_b, fg_a, bg_r, bg_g, bg_b, bg_a, modifiers_json, last_touch_stage, last_touch_effect) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)",
                params![
                    run_id,
                    frame_index,
                    cell.abs.x as i64,
                    cell.abs.y as i64,
                    cell.widget_local.x as i64,
                    cell.widget_local.y as i64,
                    cell.ch.to_string(),
                    cell.fg.r as i64,
                    cell.fg.g as i64,
                    cell.fg.b as i64,
                    cell.fg.a as i64,
                    cell.bg.r as i64,
                    cell.bg.g as i64,
                    cell.bg.b as i64,
                    cell.bg.a as i64,
                    serde_json::to_string(&cell.modifiers).unwrap_or_default(),
                    cell.last_touch.as_ref().map(|value| value.stage.clone()),
                    cell.last_touch.as_ref().and_then(|value| value.effect.clone()),
                ],
            )?;
            for (event_index, event) in cell.trace.iter().enumerate() {
                self.conn.execute(
                    "insert into probe_trace_events(run_id, frame_index, widget_x, widget_y, event_index, stage, effect, sampled_from_x, sampled_from_y, visible, before_fg_r, before_fg_g, before_fg_b, before_fg_a, before_bg_r, before_bg_g, before_bg_b, before_bg_a, after_fg_r, after_fg_g, after_fg_b, after_fg_a, after_bg_r, after_bg_g, after_bg_b, after_bg_a) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26)",
                    params![
                        run_id,
                        frame_index,
                        cell.widget_local.x as i64,
                        cell.widget_local.y as i64,
                        event_index as i64,
                        event.stage,
                        event.effect,
                        event.sampled_from.map(|value| value.x as i64),
                        event.sampled_from.map(|value| value.y as i64),
                        event.visible.map(|value| if value { 1 } else { 0 }),
                        event.before.as_ref().map(|value| value.fg.r as i64),
                        event.before.as_ref().map(|value| value.fg.g as i64),
                        event.before.as_ref().map(|value| value.fg.b as i64),
                        event.before.as_ref().map(|value| value.fg.a as i64),
                        event.before.as_ref().map(|value| value.bg.r as i64),
                        event.before.as_ref().map(|value| value.bg.g as i64),
                        event.before.as_ref().map(|value| value.bg.b as i64),
                        event.before.as_ref().map(|value| value.bg.a as i64),
                        event.after.as_ref().map(|value| value.fg.r as i64),
                        event.after.as_ref().map(|value| value.fg.g as i64),
                        event.after.as_ref().map(|value| value.fg.b as i64),
                        event.after.as_ref().map(|value| value.fg.a as i64),
                        event.after.as_ref().map(|value| value.bg.r as i64),
                        event.after.as_ref().map(|value| value.bg.g as i64),
                        event.after.as_ref().map(|value| value.bg.b as i64),
                        event.after.as_ref().map(|value| value.bg.a as i64),
                    ],
                )?;
            }
        }
        Ok(())
    }
}

// <FILE>crates/tui-vfx-probe/src/cls_probe_sqlite_store.rs</FILE> - <DESC>In-memory SQLite index for probe playback data</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
