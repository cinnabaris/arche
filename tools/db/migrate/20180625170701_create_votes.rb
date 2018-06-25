class CreateVotes < ActiveRecord::Migration[5.2]
  def change
    create_table :votes do |t|
      t.integer :point, null: false, limit: 8
      t.string :resource_type, null: false, limit: 255
      t.integer :resource_id, null: false, limit: 8
      t.timestamps
    end
    add_index :votes, :resource_type
    add_index :votes, %i[resource_type resource_id], unique: true
  end
end
