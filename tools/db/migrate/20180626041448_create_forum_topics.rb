class CreateForumTopics < ActiveRecord::Migration[5.2]
  def change
    create_table :forum_topics do |t|
      t.references :user, null: false
      t.string :lang, null: false, limit: 8
      t.string :title, null: false, limit: 255
      t.text :body, null: false
      t.string :media_type, null: false, limit: 8
      t.timestamps
    end
    add_index :forum_topics, :title
    add_index :forum_topics, :lang

    create_table :forum_topics_tags do |t|
      t.references :topic, null: false
      t.references :tag, null: false
    end
    add_index :forum_topics_tags, %i[topic_id tag_id], unique: true
  end
end
