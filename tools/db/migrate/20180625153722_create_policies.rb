class CreatePolicies < ActiveRecord::Migration[5.2]
  def change
    create_table :policies do |t|
      t.references :user, null: false
      t.string :role, null: false, limit: 255
      t.string :resource, limit: 255
      t.date :nbf, null: false
      t.date :exp, null: false
      t.timestamps
    end
    add_index :policies, :role
  end
end
