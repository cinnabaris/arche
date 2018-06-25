class CreatePolicies < ActiveRecord::Migration[5.2]
  def change
    create_table :policies do |t|
      t.references :user, null: false
      t.references :role, null: false
      t.date :nbf, null: false
      t.date :exp, null: false
      t.timestamps
    end
    add_index :policies, %i[user_id role_id], unique: true
  end
end
