class CreateCaringTopics < ActiveRecord::Migration[5.2]
  def change
    create_table :caring_topics do |t|
      t.references :member, null: false
      t.string :title, null: false, limit: 255
      t.text :body, null: false
      t.string :media_type, null: false, limit: 8
      t.timestamps
    end
    create_table :caring_topics_tags do |t|
      t.references :topic, null: false
      t.references :tag, null: false
      t.datetime :created_at, null: false
    end
    add_index :caring_topics_tags, %i[topic_id tag_id], unique: true

    create_table :caring_managers do |t|
      t.references :topic, null: false
      t.references :user, null: false
      t.datetime :created_at, null: false
    end
    add_index :caring_managers, %i[topic_id user_id], unique: true
  end
end
