class CreateLeaveWords < ActiveRecord::Migration[5.2]
  def change
    create_table :leave_words do |t|
      t.text :body, null: false
      t.string :media_type, null: false, limit: 8
      t.datetime :created_at, null: false
    end
  end
end
