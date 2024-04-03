#[derive(serde::Serialize, Debug, diesel::Selectable, diesel::Queryable)]
#[diesel(table_name = ec::schema::ec_contest_submission)]
pub struct Submission {
    id: i64,
    title: String,
    deploy_url: String,
    source_url: String,
    message: String,
    user_id: i64,
    is_winner: bool,
}

#[derive(serde::Serialize, Debug)]
pub struct Submissions {
    submissions: Vec<Submission>,
    is_admin: bool,
}

impl Submissions {
    fn get_all(conn: &mut ft_sdk::PgConnection, is_admin: bool) -> Result<Self, ec::ContestError> {
        use diesel::prelude::*;
        use ec::schema::ec_contest_submission;

        let subs: Vec<_> = ec_contest_submission::table
            .select(Submission::as_select())
            .load(conn)?;

        Ok(Self {
            submissions: subs,
            is_admin,
        })
    }

    /// get all submissions for a user
    fn get_by_user(
        conn: &mut ft_sdk::PgConnection,
        user_id: i64,
        is_admin: bool,
    ) -> Result<Self, ec::ContestError> {
        use diesel::prelude::*;
        use ec::schema::ec_contest_submission;

        let subs: Vec<_> = ec_contest_submission::table
            .filter(ec_contest_submission::user_id.eq(user_id))
            .select(Submission::as_select())
            .load(conn)?;

        Ok(Self {
            submissions: subs,
            is_admin,
        })
    }

    /// get a submission by id
    fn get_by_id(
        conn: &mut ft_sdk::PgConnection,
        id: i64,
    ) -> Result<Option<Submission>, ec::ContestError> {
        use diesel::prelude::*;
        use ec::schema::ec_contest_submission;

        Ok(ec_contest_submission::table
            .filter(ec_contest_submission::id.eq(id))
            .select(Submission::as_select())
            .first(conn)
            .optional()?)
    }
}

impl ft_sdk::Page<ec::Contest, ec::ContestError> for Submissions {
    fn page(c: &mut ec::Contest) -> Result<Self, ec::ContestError> {
        let is_admin = c.ud.email.contains("fifthtry.com") && c.ud.verified_email;

        if is_admin {
            return Self::get_all(&mut c.conn, is_admin);
        }

        return Self::get_by_user(&mut c.conn, c.ud.id, is_admin);
    }
}

#[derive(serde::Serialize, Debug, diesel::Insertable)]
#[diesel(table_name = ec::schema::ec_contest_submission)]
pub struct SubmissionPayload {
    title: String,
    deploy_url: String,
    source_url: String,
    message: String,
    is_winner: bool,
    user_id: i64,
    contest_id: i64,
}

impl ft_sdk::Action<ec::Contest, ec::ContestError> for SubmissionPayload {
    fn validate(c: &mut ec::Contest) -> Result<Self, ec::ContestError> {
        use diesel::prelude::*;
        use ec::schema::ec_contest;
        pub use ft_sdk::JsonBodyExt;

        let body = c.in_.req.json_body()?;

        let title = get_required_json_field(&body, "title")?;
        let deploy_url = get_required_json_field(&body, "deploy_url")?;
        let source_url = get_required_json_field(&body, "source_url")?;
        let message = get_required_json_field(&body, "message")?;

        // WARN: assumes a contest with id: 1 has been populated in the db
        // see: `scripts/setup-contest.py` in ft repo
        let contest_id: i64 = ec_contest::table
            .select(ec_contest::id)
            .first(&mut c.conn)
            .map_err(|e| {
                ft_sdk::println!("error: {:?}", e);
                ft_sdk::println!(
                    "Current implementation requires one \
                        manual entry to be put in `ec_contest` table."
                );

                ec::ContestError::UsageError(
                    "Current implementation requires one \
                        manual entry to be put in `ec_contest` table."
                        .to_string(),
                )
            })?;

        Ok(Self {
            title,
            deploy_url,
            source_url,
            message,
            // TODO: add method to set this
            is_winner: false,
            contest_id,
            user_id: c.ud.id,
        })
    }

    fn action(&self, c: &mut ec::Contest) -> Result<ft_sdk::ActionOutput, ec::ContestError> {
        use diesel::prelude::*;
        use ec::schema::ec_contest_submission;

        let affected = diesel::insert_into(ec_contest_submission::table)
            .values(self)
            .execute(&mut c.conn);

        ft_sdk::println!("stored submissions, affected: {:?}", affected);

        Ok(ft_sdk::ActionOutput::Redirect(
            ec::urls::contest_submissions_url(),
        ))
    }
}

pub struct DeleteSubmissionPayload {
    id: i64,
}

impl ft_sdk::Action<ec::Contest, ec::ContestError> for DeleteSubmissionPayload {
    fn validate(c: &mut ec::Contest) -> Result<Self, ec::ContestError> {
        pub use ft_sdk::JsonBodyExt;

        let body = c.in_.req.json_body()?;
        let id = get_required_json_field(&body, "id")?;
        let id = id
            .parse::<i64>()
            .map_err(|_| ec::ContestError::form_error("id", "id must be a number"))?;

        let sub = Submissions::get_by_id(&mut c.conn, id)?;

        if sub.is_none() {
            return Err(ec::ContestError::form_error("id", "submission not found"));
        }

        let sub = sub.expect("sub is not none");

        // only the user who submitted can delete
        if sub.user_id != c.ud.id {
            return Err(ec::ContestError::form_error("id", "submission not found"));
        }

        Ok(Self { id })
    }

    fn action(&self, c: &mut ec::Contest) -> Result<ft_sdk::ActionOutput, ec::ContestError> {
        use diesel::prelude::*;
        use ec::schema::ec_contest_submission;

        let affected = diesel::delete(
            ec_contest_submission::table.filter(ec_contest_submission::id.eq(self.id)),
        )
        .execute(&mut c.conn);

        ft_sdk::println!("deleted submissions, affected: {:?}", affected);

        Ok(ft_sdk::ActionOutput::Redirect(
            ec::urls::contest_submissions_url(),
        ))
    }
}

fn get_required_json_field(body: &ft_sdk::JsonBody, key: &str) -> Result<String, ec::ContestError> {
    let val = body.field::<String>(key)?.ok_or_else(|| {
        ec::ContestError::form_error(key, format!("{} is required", key).as_str())
    })?;

    if val.is_empty() {
        return Err(ec::ContestError::form_error(
            key,
            format!("{} is required", key).as_str(),
        ));
    }

    Ok(val)
}
