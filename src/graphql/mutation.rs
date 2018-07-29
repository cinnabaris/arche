use juniper::{self, FieldResult};

use super::super::plugins::{caring, forum, nut};
use super::{context::Context, H};

pub struct Mutation;

graphql_object!(
    Mutation: Context | &self | {
        //-----------------------caring---------------
        field createCaringPost(&executor, topic_id: String, method: String, body: String, media_type: String, begin: String, end: String) -> FieldResult<H> {
            gq!(executor, caring::graphql::posts::Create{
                topic_id: topic_id,
                method: method,
                body: body,
                media_type: media_type,
                begin: begin,
                end: end,
            })
        }
        field updateCaringPost(&executor, id: String, method: String, body: String, media_type: String, begin: String, end: String) -> FieldResult<H> {
            gq!(executor, caring::graphql::posts::Update{
                id: id,
                method: method,
                body: body,
                media_type: media_type,
                begin: begin,
                end: end,
            })
        }
        field removeCaringPost(&executor, id: String) -> FieldResult<H> {
            gq!(executor, caring::graphql::posts::Remove{
                id: id,
            })
        }
        field createCaringTopic(&executor, member_id: String, tag: String, name: String, gender: String, age: i32, phone: Option<String>, email: Option<String>, address: Option<String>, reason: String, media_type: String, status: String) -> FieldResult<H> {
            gq!(executor, caring::graphql::topics::Create{
                member_id: member_id,
                tag: tag,
                name: name,
                gender: gender,
                age: age,
                phone: phone,
                email: email,
                address: address,
                reason: reason,
                media_type: media_type,
                status: status,
            })
        }
        field updateCaringTopic(&executor, id: String, tag: String, name: String, gender: String, age: i32, phone: Option<String>, email: Option<String>, address: Option<String>, reason: String, media_type: String, status: String) -> FieldResult<H> {
            gq!(executor, caring::graphql::topics::Update{
                id: id,
                tag: tag,
                name: name,
                gender: gender,
                age: age,
                phone: phone,
                email: email,
                address: address,
                reason: reason,
                media_type: media_type,
                status: status,
            })
        }
        field removeCaringTopic(&executor, id: String) -> FieldResult<H> {
            gq!(executor, caring::graphql::topics::Remove{
                id: id,
            })
        }

        //-----------------------forum----------------
        field createForumTag(&executor, name: String) -> FieldResult<H> {
            gq!(executor, forum::graphql::tags::Create{
                name: name,
            })
        }
        field updateForumTag(&executor, id: String, name: String) -> FieldResult<H> {
            gq!(executor, forum::graphql::tags::Update{
                id: id,
                name: name,
            })
        }
        field removeForumTag(&executor, id: String) -> FieldResult<H> {
            gq!(executor, forum::graphql::tags::Remove{
                id: id,
            })
        }
        field createForumPost(&executor, topic_id: String, post_id: Option<String>, body: String, media_type: String) -> FieldResult<H> {
            gq!(executor, forum::graphql::posts::Create{
                topic_id: topic_id,
                post_id: post_id,
                body: body,
                media_type: media_type,
            })
        }
        field updateForumPost(&executor, id: String, body: String, media_type: String) -> FieldResult<H> {
            gq!(executor, forum::graphql::posts::Update{
                id: id,
                body: body,
                media_type: media_type,
            })
        }
        field removeForumPost(&executor, id: String) -> FieldResult<H> {
            gq!(executor, forum::graphql::posts::Remove{
                id: id,
            })
        }
        field createForumTopic(&executor, title: String, body: String, media_type: String, tags: Vec<String>) -> FieldResult<H> {
            gq!(executor, forum::graphql::topics::Create{
                title: title,
                body: body,
                media_type: media_type,
                tags: tags,
            })
        }
        field updateForumTopic(&executor, id: String, title: String, body: String, media_type: String, tags: Vec<String>) -> FieldResult<H> {
            gq!(executor, forum::graphql::topics::Update{
                id: id,
                title: title,
                body: body,
                media_type: media_type,
                tags: tags,
            })
        }
        field removeForumTopic(&executor, id: String) -> FieldResult<H> {
            gq!(executor, forum::graphql::topics::Remove{
                id: id,
            })
        }
        //-----------------------nut----------------
        field createMember(&executor, nick_name: String, real_name: String, gender: String, birthday: String, phone: Option<String>, email: Option<String>, address: Option<String>, line: Option<String>, wechat: Option<String>, skype: Option<String>, weibo: Option<String>, facebook: Option<String>) -> FieldResult<H> {
            gq!(executor, nut::graphql::members::Create{
                nick_name: nick_name.to_lowercase().trim().to_string(),
                real_name: real_name,
                gender: gender,
                birthday: birthday,
                phone: phone,
                email: email,
                address: address,
                line: line,
                wechat: wechat,
                skype: skype,
                weibo: weibo,
                facebook: facebook,
            })
        }
        field updateMember(&executor, id: String, real_name: String, gender: String, birthday: String, phone: Option<String>, email: Option<String>, address: Option<String>, line: Option<String>, wechat: Option<String>, skype: Option<String>, weibo: Option<String>, facebook: Option<String>) -> FieldResult<H> {
            gq!(executor, nut::graphql::members::Update{
                id: id,
                real_name: real_name,
                gender: gender,
                birthday: birthday,
                phone: phone,
                email: email,
                address: address,
                line: line,
                wechat: wechat,
                skype: skype,
                weibo: weibo,
                facebook: facebook,
            })
        }
        field removeMember(&executor, id: String) -> FieldResult<H> {
            gq!(executor, nut::graphql::members::Remove{
                id: id,
            })
        }

        field createFriendLink(&executor, title: String, home: String, logo: String, position: i32) -> FieldResult<H> {
            gq!(executor, nut::graphql::friend_links::Create{
                title: title,
                home: home,
                logo: logo,
                position: position,
            })
        }
        field updateFriendLink(&executor, id: String, title: String, home: String, logo: String, position: i32) -> FieldResult<H> {
            gq!(executor, nut::graphql::friend_links::Update{
                id: id,
                title: title,
                home: home,
                logo: logo,
                position: position,
            })
        }
        field removeFriendLink(&executor, id: String) -> FieldResult<H> {
            gq!(executor, nut::graphql::friend_links::Remove{
                id: id,
            })
        }

        field createCard(&executor, title: String, body: String, media_type: String, action: String, href: String, logo: String, loc: String, position: i32) -> FieldResult<H> {
            gq!(executor, nut::graphql::cards::Create{
                title: title,
                body: body,
                media_type: media_type,
                action: action,
                href: href,
                logo: logo,
                loc: loc,
                position: position,
            })
        }
        field updateCard(&executor, id: String, title: String, body: String, media_type: String, action: String, href: String, logo: String, loc: String, position: i32) -> FieldResult<H> {
            gq!(executor, nut::graphql::cards::Update{
                id: id,
                title: title,
                body: body,
                media_type: media_type,
                action: action,
                href: href,
                logo: logo,
                loc: loc,
                position: position,
            })
        }
        field removeCard(&executor, id: String) -> FieldResult<H> {
            gq!(executor, nut::graphql::cards::Remove{
                id: id,
            })
        }

        field createLink(&executor, label: String, href: String, loc: String, x: i32, y: i32) -> FieldResult<H> {
            gq!(executor, nut::graphql::links::Create{
                label: label,
                href: href,
                loc: loc,
                x: x,
                y: y,
            })
        }
        field updateLink(&executor, id: String, label: String, href: String, loc: String, x: i32, y: i32) -> FieldResult<H> {
            gq!(executor, nut::graphql::links::Update{
                id: id,
                label: label,
                href: href,
                loc: loc,
                x: x,
                y: y,
            })
        }
        field removeLink(&executor, id: String) -> FieldResult<H> {
            gq!(executor, nut::graphql::links::Remove{
                id: id,
            })
        }

        field updateSiteSmtp(&executor, host: String, port: i32, user: String, password:String) -> FieldResult<H> {
            gq!(executor, nut::graphql::site::mutation::UpdateSmtp{
                host: host,
                port: port,
                user: user,
                password: password,
            })
        }
        field updateSiteSeo(&executor, google_site_verify_code: String, baidu_site_verify_code: String) -> FieldResult<H> {
            gq!(executor, nut::graphql::site::mutation::UpdateSeo{
                google_site_verify_code: google_site_verify_code,
                baidu_site_verify_code: baidu_site_verify_code,
            })
        }
        field updateSiteAuthor(&executor, name: String, email: String) -> FieldResult<H> {
            gq!(executor, nut::graphql::site::mutation::UpdateAuthor{
                name: name,
                email: email,
            })
        }
        field updateSiteInfo(&executor, title: String, subhead:String, keywords:String, description:String, copyright:String) -> FieldResult<H> {
            gq!(executor, nut::graphql::site::mutation::UpdateInfo{
                title: title,
                subhead: subhead,
                keywords: keywords,
                description: description,
                copyright: copyright,
            })
        }

        field removeLocale(&executor, id: String) -> FieldResult<H> {
            gq!(executor, nut::graphql::locales::Remove{
                id: id,
            })
        }
        field updateLocale(&executor, code: String, message: String) -> FieldResult<H> {
            gq!(executor, nut::graphql::locales::Update{
                code: code,
                message: message,
            })
        }

        field removeLeaveWord(&executor, id: String) -> FieldResult<H> {
            gq!(executor, nut::graphql::leave_words::Remove{
                id: id,
            })
        }
        field createLeaveWord(&executor, media_type: String, body: String) -> FieldResult<H> {
            gq!(executor, nut::graphql::leave_words::Create{
                media_type: media_type,
                body: body,
            })
        }

        field lockUser(&executor, id: String) -> FieldResult<H> {
            gq!(executor, nut::graphql::users::mutation::Lock{
                id: id,
            })
        }
        field updateUserPolicy(&executor, user: String, policies: String, nbf:String, exp:String) -> FieldResult<H> {
            gq!(executor, nut::graphql::users::mutation::UpdatePolicy{
                user: user,
                policies: policies,
                nbf: nbf,
                exp: exp,
            })
        }
        field updateUserProfile(&executor, name: String, logo: String) -> FieldResult<H> {
            gq!(executor, nut::graphql::users::mutation::UpdateProfile{
                name: name,
                logo: logo,
            })
        }
        field changeUserPassword(&executor, current_password: String, new_password: String) -> FieldResult<H> {
            gq!(executor, nut::graphql::users::mutation::ChangePassword{
                current_password: current_password,
                new_password: new_password,
            })
        }
        field signOutUser(&executor) -> FieldResult<H> {
            ge!(nut::graphql::users::mutation::sign_out(executor.context()))
        }

        field signInUserByEmail(&executor, email: String, password: String) -> FieldResult<nut::graphql::users::models::SignIn> {
            gq!(executor, nut::graphql::users::mutation::SignInByEmail{
                email: email.to_lowercase().trim().to_string(),
                password: password,
            })
        }
        field resetUserPassword(&executor, token: String, password: String) -> FieldResult<H> {
            gq!(executor, nut::graphql::users::mutation::ResetPassword{
                token: token,
                password: password,
            })
        }
        field unlockUser(&executor, token: String) -> FieldResult<H> {
            gq!(executor, nut::graphql::users::mutation::Unlock{
                token: token,
            })
        }
        field confirmUser(&executor, token: String) -> FieldResult<H> {
            gq!(executor, nut::graphql::users::mutation::Confirm{
                token: token,
            })
        }
        field signUpUser(&executor, name: String, email:String, password:String) -> FieldResult<H> {
            gq!(executor, nut::graphql::users::mutation::SignUp{
                name: name,
                email: email.to_lowercase().trim().to_string(),
                password: password,
            })
        }

        field install(&executor, name: String, email:String, password:String) -> FieldResult<H> {
            gq!(executor, nut::graphql::home::Install{
                name: name,
                email: email.to_lowercase().trim().to_string(),
                password: password,
            })
        }
    }
);
