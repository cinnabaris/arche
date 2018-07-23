class CreateCaringTopics < ActiveRecord::Migration[5.2]
  def change
    create_table :caring_topics do |t|
      t.references :user, null: false
      t.references :member, null: false
      t.string :tag, null: false, limit: 32
      t.string :name, null: false, limit: 36
      t.string :gender, null: false, limit: 1
      t.integer :age, null: false, limit: 1
      t.string :phone,  limit: 255
      t.string :email,  limit: 255
      t.string :address, limit: 255
      t.text :reason, null: false
      t.string :media_type, null: false, limit: 8
      t.string :status, null: false, limit: 16
      t.timestamps
    end
    add_index :caring_topics, :tag
    add_index :caring_topics, :name
    add_index :caring_topics, :gender
    add_index :caring_topics, :status
  end
end
